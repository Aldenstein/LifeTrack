// Module de gestion des erreurs centralisé
//
// Description (FR):
// Ce module définit le type d'erreur applicatif `AppError` avec
// tous ses variants, et l'implémentation de `IntoResponse` pour
// convertir automatiquement vers des réponses HTTP standardisées.
// Toutes les fonctions retournent `Result<T>` qui est `std::result::Result<T, AppError>`.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sqlx::Error as SqlxError;
use serde::Serialize;
use thiserror::Error;

/// Type d'erreur applicatif centralisé
/// Tous les handlers et fonctions retournent `Result<T>` = `std::result::Result<T, AppError>`
#[derive(Debug, Error)]
pub enum AppError {
    /// Erreur de configuration (variable d'environnement manquante, etc.)
    #[error("Erreur de configuration: {0}")]
    Config(String),

    /// Erreur de validation côté client (400)
    #[error("Requête invalide: {0}")]
    BadRequest(String),

    /// Erreur provenant de la base de données (sqlx::Error)
    #[error("Erreur base de données")]
    Database(SqlxError),

    /// Ressource non trouvée (404)
    #[error("Ressource non trouvée")]
    NotFound,

    /// Conflit de données (409) - par exemple, email déjà utilisé
    #[error("Conflit de données: {0}")]
    Conflict(String),

    /// Authentification échouée ou non autorisée (401)
    #[error("Non autorisé: {0}")]
    Unauthorized(String),

    /// Erreur interne générique (500)
    #[error("Erreur interne")]
    Internal(#[from] anyhow::Error),
}

/// Structure de réponse d'erreur JSON
#[derive(Serialize)]
struct ErrorBody {
    message: String,
}

/// Implémentation de `IntoResponse` pour convertir `AppError` en réponse HTTP
/// Cette trait est utilisée automatiquement par Axum pour les handlers retournant `Result<T, AppError>`
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, msg) = match &self {
            AppError::Config(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Database(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Erreur base de données".into(),
            ),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Ressource non trouvée".into()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Erreur interne".into()),
        };

        if matches!(self, AppError::Config(_) | AppError::Database(_) | AppError::Internal(_)) {
            eprintln!("HTTP {} returned from AppError: {:?}", status.as_u16(), self);
        }

        let body = Json(ErrorBody { message: msg });
        (status, body).into_response()
    }
}

impl From<SqlxError> for AppError {
    fn from(error: SqlxError) -> Self {
        match error {
            SqlxError::RowNotFound => AppError::NotFound,
            SqlxError::Database(db_error) => {
                let code = db_error.code().map(|value| value.to_string());

                if matches!(code.as_deref(), Some("1452")) {
                    AppError::NotFound
                } else if matches!(code.as_deref(), Some("1062")) {
                    AppError::Conflict(format!("Violation de contrainte unique: {}", db_error.message()))
                } else {
                    AppError::Database(SqlxError::Database(db_error))
                }
            }
            other => AppError::Database(other),
        }
    }
}

/// Alias de type pour simplifier les signatures: `Result<T>` = `std::result::Result<T, AppError>`
pub type Result<T> = std::result::Result<T, AppError>;
