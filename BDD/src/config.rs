use std::env;
use crate::errors::{AppError, Result};

/// Configuration globale de l'application
/// Contient les paramètres de connexion à la base de données et les secrets JWT
pub struct DbConfig {
    /// URL de connexion à la base de données MySQL/MariaDB
    pub url: String,
    /// Hôte d'écoute HTTP
    pub api_host: String,
    /// Port d'écoute HTTP
    pub api_port: u16,
    /// Clé secrète pour signer et vérifier les tokens JWT
    pub jwt_secret: String,
}

/// Charge la configuration depuis les variables d'environnement
/// Lit le fichier .env (en cherchant d'abord le répertoire courant, puis ../.env)
/// et récupère DATABASE_URL (obligatoire), API_HOST/API_PORT (optionnels) et JWT_SECRET (optionnel)
///
/// # Erreurs
/// Retourne `AppError::Config` si `DATABASE_URL` n'est pas définie
pub fn load_config() -> Result<DbConfig> {
    // 1. Essayer d'abord .env dans le répertoire courant
    if dotenvy::dotenv().is_err() {
        // 2. En fallback, essayer ../.env
        let _ = dotenvy::from_filename("../.env");
    }

    // Récupérer l'URL de la base de données (obligatoire)
    let url = env::var("DATABASE_URL")
        .map_err(|_| AppError::Config("DATABASE_URL manquante".into()))?;

    let api_host = env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

    let api_port = env::var("API_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .map_err(|_| AppError::Config("API_PORT invalide".into()))?;

    // Récupérer le secret JWT (optionnel, utilise une valeur par défaut en dev)
    let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-me-in-production".to_string());

    Ok(DbConfig {
        url,
        api_host,
        api_port,
        jwt_secret,
    })
}