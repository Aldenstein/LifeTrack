// Module utilitaires
//
// Description (FR):
// Fonctions utilitaires réutilisables par les handlers et la couche DB.
// Actuellement contient des helpers de parsing/validation (dates), mais
// peut être étendu pour d'autres utilitaires transverses.

use chrono::NaiveDate;
use crate::errors::{AppError, Result};

/// Parse une date au format YYYY-MM-DD
/// Valide le format et retourne une erreur si invalide
///
/// # Erreurs
/// Retourne `AppError::BadRequest` si le format de date est invalide
pub fn parse_date(date: &str) -> Result<NaiveDate> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| AppError::BadRequest(format!("Invalid date format '{date}', expected YYYY-MM-DD")))
}
