mod api;
mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod utils;

use axum::serve;
use crate::config::load_config;
use crate::db::{connect_db, DbPool};
use crate::routes::init_routes;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // 1. Charger la config
    let cfg = load_config();

    // 2. Se connecter à la base (création du pool)
    let pool: DbPool = connect_db(&cfg).await;

    println!("Connexion à la base réussie !");

    // 3. Construire les routes HTTP
    let app = init_routes(pool);

    // 4. Démarrer le serveur HTTP
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Impossible de démarrer le listener HTTP");

    println!("Serveur lancé sur http://127.0.0.1:3000");
    serve(listener, app)
        .await
        .expect("Erreur pendant l'exécution du serveur HTTP");
}