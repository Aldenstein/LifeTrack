use axum::{routing::get, Router};

use crate::api::{today_dashboard, user_profile};
use crate::db::DbPool;

pub fn init_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/users/:user_id/profile", get(user_profile))
        .route("/users/:user_id/dashboard/today", get(today_dashboard))
        .with_state(pool)
}