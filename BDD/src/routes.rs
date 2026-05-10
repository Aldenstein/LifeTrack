use axum::{routing::get, Router};

use crate::api::{
    today_dashboard, user_profile, latest_module_values, active_alerts_and_reminders,
    health_check, api_info
};
use crate::db::DbPool;

pub fn init_routes(pool: DbPool) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/api/info", get(api_info))
        .route("/users/:user_id/profile", get(user_profile))
        .route("/users/:user_id/dashboard/today", get(today_dashboard))
        .route("/users/:user_id/latest-module-values", get(latest_module_values))
        .route("/users/:user_id/alerts-reminders", get(active_alerts_and_reminders))
        .with_state(pool)
}