//! # LifeTrack API Backend
//! Serveur REST pour la gestion du suivi de vie personnelle
//! Inclut authentification par passphrase, chiffrement Zero-Knowledge et dashboards

/// Gestion des endpoints HTTP
pub mod api;
/// GET endpoints pour les fonctions de lecture
pub mod api_get_endpoints;
/// Authentification par JWT et passphrase
pub mod auth;
/// Configuration globale (variables d'environnement)
pub mod config;
/// Fonctions d'accès à la base de données
pub mod db;
/// Gestion des erreurs personnalisées
pub mod errors;
/// Gestion des mots de passe (Argon2, PBKDF2)
pub mod password;
/// Structures de données (Request/Response, DB models)
pub mod models;
/// Définition des routes HTTP
pub mod routes;
/// Fonctions utilitaires
pub mod utils;
/// Hashing et dérivation de clés (Argon2, PBKDF2)
pub mod password;

/// Réexporte publiquement tous les modèles
pub use models::*;
/// Réexporte publiquement les fonctions DB
pub use db::*;
