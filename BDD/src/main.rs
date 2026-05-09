mod api;
mod auth;
mod config;
mod db;
mod errors;
mod models;
mod routes;
mod utils;

use crate::config::load_config;
use crate::db::{connect_db, DbPool};

#[tokio::main]
async fn main() {
    // 1. Charger la config
    let cfg = load_config();

    // 2. Se connecter à la base (création du pool)
    let pool: DbPool = connect_db(&cfg).await;

    println!("Connexion à la base réussie !");
}