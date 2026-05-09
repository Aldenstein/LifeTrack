use sqlx::{Pool, MySql};
use sqlx::mysql::MySqlPoolOptions;

use crate::config::DbConfig;

pub type DbPool = Pool<MySql>;

pub async fn connect_db(cfg: &DbConfig) -> DbPool {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&cfg.url)
        .await
        .expect("Impossible de se connecter à la base MariaDB/MySQL")
}