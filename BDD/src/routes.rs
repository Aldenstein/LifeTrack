// Module de définition des routes HTTP
// Sépare les routes publiques (authentification) des routes protégées (JWT requis)

use axum::{routing::{get, post}, Router, middleware};

use crate::api::{
    active_alerts_and_reminders, api_info, create_account_endpoint, create_finance_type_endpoint,
    create_habit_category_endpoint, create_habit_endpoint, create_mood_type_endpoint,
    create_planned_expense_endpoint, create_sobriety_period_endpoint, create_sport_type_endpoint,
    create_todo_endpoint, create_transaction_endpoint, create_user_endpoint, health_check,
    latest_module_values, log_alcohol_consumption_endpoint, log_body_measurement_endpoint,
    log_breathing_session_endpoint, log_hydration_endpoint, log_meal_endpoint, log_mood_endpoint,
    log_sleep_endpoint, log_sport_session_endpoint, mark_habit_complete_endpoint,
    today_dashboard, user_profile, get_current_user,
    // Zero-Knowledge
    save_encrypted_entry_endpoint,
    get_encrypted_entries_endpoint,
    get_all_encrypted_entries_endpoint,
};
use crate::auth::{register_endpoint, login_endpoint, derive_keys_endpoint, validate_jwt};
use crate::db::DbPool;

pub fn init_routes(pool: DbPool, jwt_secret: String) -> Router {
    let state = (pool.clone(), jwt_secret.clone());
    let protected_state = (pool.clone(), jwt_secret);
    
    Router::new()
        // ── Routes publiques ────────────────────────────────────────────────
        .route("/health", get(health_check))
        .route("/api/info", get(api_info))
        .route("/auth/register", post(register_endpoint))
        .route("/auth/login", post(login_endpoint))
        .route("/auth/derive-keys", post(derive_keys_endpoint))
        .with_state(state)
        
        // ── Routes protégées par JWT ─────────────────────────────────────────
        .route("/users/me/:user_id", get(get_current_user))
        .route("/users/:user_id/profile", get(user_profile))
        .route("/users/:user_id/dashboard/today", get(today_dashboard))
        .route("/users/:user_id/latest-module-values", get(latest_module_values))
        .route("/users/:user_id/alerts-reminders", get(active_alerts_and_reminders))
        .route("/users", post(create_user_endpoint))
        .route("/users/:user_id/accounts", post(create_account_endpoint))
        .route("/finance/types", post(create_finance_type_endpoint))
        .route("/users/:user_id/transactions", post(create_transaction_endpoint))
        .route("/users/:user_id/planned-expenses", post(create_planned_expense_endpoint))
        .route("/habits/categories", post(create_habit_category_endpoint))
        .route("/users/:user_id/habits", post(create_habit_endpoint))
        .route(
            "/users/:user_id/habits/:habit_id/complete",
            post(mark_habit_complete_endpoint),
        )
        .route(
            "/users/:user_id/sobriety-periods",
            post(create_sobriety_period_endpoint),
        )
        .route("/mood/types", post(create_mood_type_endpoint))
        .route("/users/:user_id/moods", post(log_mood_endpoint))
        .route("/users/:user_id/hydration", post(log_hydration_endpoint))
        .route("/users/:user_id/sleep", post(log_sleep_endpoint))
        .route("/users/:user_id/meals", post(log_meal_endpoint))
        .route(
            "/users/:user_id/body-measurements",
            post(log_body_measurement_endpoint),
        )
        .route("/sport/types", post(create_sport_type_endpoint))
        .route("/users/:user_id/sport-sessions", post(log_sport_session_endpoint))
        .route(
            "/users/:user_id/breathing-sessions",
            post(log_breathing_session_endpoint),
        )
        .route(
            "/users/:user_id/alcohol-consumptions",
            post(log_alcohol_consumption_endpoint),
        )
        .route("/users/:user_id/todos", post(create_todo_endpoint))
        // ── Zero-Knowledge routes ────────────────────────────────────────────
        .route(
            "/users/:user_id/encrypted",
            post(save_encrypted_entry_endpoint).get(get_encrypted_entries_endpoint),
        )
        .route(
            "/users/:user_id/encrypted/all",
            get(get_all_encrypted_entries_endpoint),
        )
        .layer(middleware::from_fn_with_state(protected_state, validate_jwt))
        .with_state(pool)
}
