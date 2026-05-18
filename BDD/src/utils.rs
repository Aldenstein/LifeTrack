use axum::http::StatusCode;
use axum::Json;
use chrono::NaiveDate;

use crate::models::ApiError;

/// Parse une date au format YYYY-MM-DD
/// Valide le format et retourne une erreur HTTP 400 si invalide
/// Centralisée ici pour éviter la duplication de code
pub fn parse_date(date: &str) -> Result<NaiveDate, (StatusCode, Json<ApiError>)> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                message: format!("Invalid date format '{date}', expected YYYY-MM-DD"),
            }),
        )
    })
}
