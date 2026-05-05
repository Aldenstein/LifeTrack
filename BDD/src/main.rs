mod api;
mod sql_to_json;

use dotenvy::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let api_host = env::var("API_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let api_port = env::var("API_PORT").unwrap_or_else(|_| "8080".to_string());
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "DATABASE_URL non définie".to_string());

    println!("API: {}:{}", api_host, api_port);
    println!("DB: {}", database_url);

    // Décommente si ces fonctions existent dans tes fichiers :
    // api::start_api();
    // sql_to_json::convert();
}
