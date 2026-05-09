use std::env;

pub struct DbConfig {
    pub url: String,
}

pub fn load_config() -> DbConfig {
    let _ = dotenvy::from_filename("../.env");

    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    DbConfig { url }
}