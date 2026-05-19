//! Point d'entrée du serveur LifeTrack API
//! Initialise la configuration, connecte la base de données et démarre le serveur HTTP
//!
//! Description (FR):
//! Ce binaire initialise la configuration via `load_config`, établit
//! le pool de connexions DB, construit la `Router` via `init_routes`
//! et démarre le listener TCP sur `127.0.0.1:3000`.

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
use crate::routes::init_routes;
use tokio::net::TcpListener;
use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

/// Point d'entrée principal asynchrone
/// Lance le serveur HTTP sur http://127.0.0.1:3000
#[tokio::main]
async fn main() {
    // Étape 1: Charger les variables de configuration (.env)
    let cfg = load_config();
    let jwt_secret = cfg.jwt_secret.clone();

    // Étape 2: Établir la connexion à la base de données et créer le pool de connexions
    let pool: DbPool = connect_db(&cfg).await;
    println!("✓ Connexion à la base de données réussie !");

    // Étape 3: Construire l'application Axum avec toutes les routes
    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
    .allow_headers(Any);

    let app = init_routes(pool, jwt_secret).layer(cors);

    // Étape 4: Démarrer le serveur TCP qui écoute les requêtes HTTP
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Impossible de démarrer le listener TCP sur 127.0.0.1:3000");

    println!("🚀 Serveur LifeTrack lancé sur http://127.0.0.1:3000");
    println!("   Authentification: POST /auth/register ou /auth/login");
    println!("   Zero-Knowledge: POST /users/:user_id/encrypted");
    
    // Servir indéfiniment les requêtes entrantes
    serve(listener, app)
        .await
        .expect("Erreur critique pendant l'exécution du serveur HTTP");
}