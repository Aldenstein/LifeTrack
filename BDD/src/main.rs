//! Point d'entrée du serveur LifeTrack API
//! Initialise la configuration, connecte la base de données et démarre le serveur HTTP
//!
//! Description (FR):
//! Ce binaire initialise la configuration via `load_config`, établit
//! le pool de connexions DB, construit la `Router` via `init_routes`
//! et démarre le listener TCP sur l'hôte/port fournis par la configuration.

mod api;
mod api_get_endpoints;
mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod utils;
mod password;

use axum::serve;
use crate::config::load_config;
use crate::db::{connect_db, DbPool};
use crate::errors::AppError;
use crate::routes::init_routes;
use tokio::net::TcpListener;
use axum::http::Method;
use tower_http::cors::{Any, CorsLayer};

/// Point d'entrée principal asynchrone
/// Lance le serveur HTTP sur l'hôte/port configurés
#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Étape 1: Charger les variables de configuration (.env)
    let cfg = load_config()?;
    let jwt_secret = cfg.jwt_secret.clone();

    // Étape 2: Établir la connexion à la base de données et créer le pool de connexions
    let pool: DbPool = connect_db(&cfg).await?;
    println!("✓ Connexion à la base de données réussie !");

    // Étape 3: Construire l'application Axum avec toutes les routes
    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
    .allow_headers(Any);

    let app = init_routes(pool, jwt_secret).layer(cors);

    // Étape 4: Démarrer le serveur TCP qui écoute les requêtes HTTP
    let bind_address = format!("{}:{}", cfg.api_host, cfg.api_port);
    let listener = TcpListener::bind(&bind_address)
        .await
        .map_err(|error| AppError::Internal(anyhow::anyhow!("Impossible de démarrer le listener TCP sur {}: {}", bind_address, error)))?;

    println!("🚀 Serveur LifeTrack lancé sur http://{}", bind_address);
    println!("   Authentification: POST /auth/register ou /auth/login");
    println!("   Zero-Knowledge: POST /users/:user_id/encrypted");
    
    // Servir indéfiniment les requêtes entrantes
    serve(listener, app)
        .await
        .map_err(|error| AppError::Internal(anyhow::anyhow!("Erreur critique pendant l'exécution du serveur HTTP: {}", error)))?;

    Ok(())
}