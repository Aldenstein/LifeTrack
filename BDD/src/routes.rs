// Module de définition des routes HTTP
//
// Description (FR):
// Ce fichier centralise la construction de la `Router` Axum utilisée par
// l'application. Il expose la fonction `init_routes` qui assemble toutes
// les routes publiques et protégées, associe les handlers définis dans
// `api` et `api_get_endpoints`, et applique le middleware JWT pour
// sécuriser les routes nécessitant une authentification.
//
// Conventions:
// - Routes publiques: endpoints d'authentification et informations générales.
// - Routes protégées: toutes les routes manipulant des données utilisateur
//   (avec `validate_jwt` middleware).

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

use crate::api_get_endpoints::*;

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
        .route("/users/:user_id/accounts", post(create_account_endpoint).get(get_user_accounts_endpoint))
        .route("/finance/types", post(create_finance_type_endpoint).get(get_finance_types_endpoint))
        .route("/users/:user_id/transactions", post(create_transaction_endpoint))
        .route("/users/:user_id/planned-expenses", post(create_planned_expense_endpoint).get(get_planned_expenses_endpoint))
        .route("/habits/categories", post(create_habit_category_endpoint).get(get_habit_categories_endpoint))
        .route("/users/:user_id/habits", post(create_habit_endpoint))
        .route(
            "/users/:user_id/habits/:habit_id/complete",
            post(mark_habit_complete_endpoint),
        )
        .route(
            "/users/:user_id/sobriety-periods",
            post(create_sobriety_period_endpoint),
        )
        .route("/mood/types", post(create_mood_type_endpoint).get(get_mood_types_endpoint))
        .route("/users/:user_id/moods", post(log_mood_endpoint))
        .route("/users/:user_id/hydration", post(log_hydration_endpoint))
        .route("/users/:user_id/sleep", post(log_sleep_endpoint))
        .route("/users/:user_id/meals", post(log_meal_endpoint))
        .route(
            "/users/:user_id/body-measurements",
            post(log_body_measurement_endpoint),
        )
        .route("/sport/types", post(create_sport_type_endpoint).get(get_sport_types_endpoint))
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
        
        // ── FINANCE GET routes ─────────────────────────────────────────────
        .route("/users/:user_id/account-balances", get(get_account_balances_endpoint))
        .route("/users/:user_id/transactions/period", get(get_transactions_by_period_endpoint))
        .route("/users/:user_id/transactions/account/:account_id", get(get_transactions_by_account_endpoint))
        .route("/users/:user_id/transactions/type/:type_id", get(get_transactions_by_type_endpoint))
        .route("/users/:user_id/income/period", get(get_income_total_by_period_endpoint))
        .route("/users/:user_id/expenses/period", get(get_expense_total_by_period_endpoint))
        .route("/users/:user_id/balance/period", get(get_net_balance_by_period_endpoint))
        .route("/users/:user_id/planned-expenses/upcoming", get(get_upcoming_planned_expenses_endpoint))
        .route("/users/:user_id/top-expenses", get(get_top_expense_types_endpoint))
        .route("/users/:user_id/balance-history", get(get_balance_history_endpoint))
        
        // ── HABIT GET routes ───────────────────────────────────────────────
        .route("/users/:user_id/habits/active", get(get_active_habits_endpoint))
        .route("/users/:user_id/habits/positive", get(get_positive_habits_endpoint))
        .route("/users/:user_id/habits/negative", get(get_negative_habits_endpoint))
        .route("/users/:user_id/habits/today", get(get_today_habits_endpoint))
        .route("/users/:user_id/habits/completed-today", get(get_completed_habits_today_endpoint))
        .route("/users/:user_id/habits/pending-today", get(get_pending_habits_today_endpoint))
        .route("/users/:user_id/habits/summary-today", get(get_today_habit_summary_endpoint))
        .route("/users/:user_id/habits/score-today", get(get_today_habit_score_endpoint))
        .route("/users/:user_id/habits/score-weekly", get(get_weekly_habit_score_endpoint))
        .route("/users/:user_id/habits/:habit_id/history", get(get_habit_history_endpoint))
        .route("/users/:user_id/habits/:habit_id/completion-rate", get(get_habit_completion_rate_endpoint))
        .route("/users/:user_id/habits/most-consistent", get(get_most_consistent_habits_endpoint))
        .route("/users/:user_id/habits/least-consistent", get(get_least_consistent_habits_endpoint))
        
        // ── SOBRIETY GET routes ────────────────────────────────────────────
        .route("/users/:user_id/sobriety/current", get(get_current_sobriety_period_endpoint))
        .route("/users/:user_id/sobriety/current-duration", get(get_current_sobriety_duration_endpoint))
        .route("/users/:user_id/sobriety/history", get(get_sobriety_history_endpoint))
        .route("/users/:user_id/sobriety/total-duration", get(get_total_sobriety_duration_endpoint))
        .route("/users/:user_id/sobriety/stats", get(get_sobriety_stats_by_period_endpoint))
        
        // ── MOOD GET routes ────────────────────────────────────────────────
        .route("/users/:user_id/moods/today", get(get_today_mood_endpoint))
        .route("/users/:user_id/moods/by-date", get(get_mood_by_date_endpoint))
        .route("/users/:user_id/moods/monthly", get(get_monthly_moods_endpoint))
        .route("/users/:user_id/moods/most-frequent", get(get_most_frequent_mood_endpoint))
        .route("/users/:user_id/moods/distribution", get(get_mood_distribution_by_period_endpoint))
        
        // ── HYDRATION GET routes ───────────────────────────────────────────
        .route("/users/:user_id/hydration/today", get(get_today_hydration_endpoint))
        .route("/users/:user_id/hydration/goal-today", get(get_today_hydration_goal_endpoint))
        .route("/users/:user_id/hydration/history", get(get_hydration_history_endpoint))
        .route("/users/:user_id/hydration/today-total", get(get_today_water_total_endpoint))
        .route("/users/:user_id/hydration/goal-progress", get(get_hydration_goal_progress_endpoint))
        .route("/users/:user_id/hydration/goal-reached-days", get(get_hydration_goal_reached_days_endpoint))
        .route("/users/:user_id/hydration/goal-missed-days", get(get_hydration_goal_missed_days_endpoint))
        .route("/users/:user_id/hydration/average-weekly", get(get_weekly_hydration_average_endpoint))
        .route("/users/:user_id/hydration/average-monthly", get(get_monthly_hydration_average_endpoint))
        .route("/users/:user_id/hydration/goal-history", get(get_hydration_goal_history_endpoint))
        
        // ── SLEEP GET routes ───────────────────────────────────────────────
        .route("/users/:user_id/sleep/latest", get(get_latest_sleep_entry_endpoint))
        .route("/users/:user_id/sleep/today", get(get_today_sleep_endpoint))
        .route("/users/:user_id/sleep/history", get(get_sleep_history_endpoint))
        .route("/users/:user_id/sleep/restful", get(get_restful_sleep_entries_endpoint))
        .route("/users/:user_id/sleep/non-restful", get(get_non_restful_sleep_entries_endpoint))
        .route("/users/:user_id/sleep/average-weekly", get(get_weekly_sleep_average_endpoint))
        .route("/users/:user_id/sleep/average-monthly", get(get_monthly_sleep_average_endpoint))
        .route("/users/:user_id/sleep/short/:min_duration", get(get_short_sleep_entries_endpoint))
        
        // ── MEAL GET routes ────────────────────────────────────────────────
        .route("/users/:user_id/meals/today", get(get_today_meals_endpoint))
        .route("/users/:user_id/meals/period", get(get_meals_by_period_endpoint))
        .route("/users/:user_id/meals/latest", get(get_latest_meal_endpoint))
        .route("/users/:user_id/nutrition/calories-today", get(get_today_calorie_total_endpoint))
        .route("/users/:user_id/nutrition/protein-today", get(get_today_protein_total_endpoint))
        .route("/users/:user_id/nutrition/carbs-today", get(get_today_carb_total_endpoint))
        .route("/users/:user_id/nutrition/fats-today", get(get_today_fat_total_endpoint))
        .route("/users/:user_id/nutrition/macro-distribution", get(get_daily_macro_distribution_endpoint))
        .route("/users/:user_id/nutrition/history", get(get_nutrition_history_endpoint))
        .route("/users/:user_id/nutrition/calories-average-weekly", get(get_weekly_calorie_average_endpoint))
        .route("/users/:user_id/nutrition/calories-average-monthly", get(get_monthly_calorie_average_endpoint))
        
        // ── BODY MEASUREMENT GET routes ────────────────────────────────────
        .route("/users/:user_id/body-measurements/latest", get(get_latest_body_measurement_endpoint))
        .route("/users/:user_id/weight/history", get(get_weight_history_endpoint))
        .route("/users/:user_id/weight/chart", get(get_weight_chart_data_endpoint))
        .route("/users/:user_id/weight/progress", get(get_weight_progress_endpoint))
        .route("/users/:user_id/health/bmi-current", get(get_current_bmi_endpoint))
        .route("/users/:user_id/health/metrics", get(get_health_derived_metrics_endpoint))
        
        // ── SPORT GET routes ───────────────────────────────────────────────
        .route("/sport/types", get(get_sport_types_endpoint))
        .route("/users/:user_id/sport-sessions/today", get(get_today_sport_sessions_endpoint))
        .route("/users/:user_id/sport-sessions/period", get(get_sport_sessions_by_period_endpoint))
        .route("/users/:user_id/sport-sessions/latest", get(get_latest_sport_session_endpoint))
        .route("/users/:user_id/sport/duration-by-period", get(get_total_sport_duration_by_period_endpoint))
        .route("/users/:user_id/sport/count-by-period", get(get_sport_session_count_by_period_endpoint))
        .route("/users/:user_id/sport/calories-burned", get(get_burned_calories_by_period_endpoint))
        .route("/users/:user_id/sport/sessions-by-type/:sport_type_id", get(get_sport_sessions_by_type_endpoint))
        .route("/users/:user_id/sport/most-practiced", get(get_most_practiced_sport_endpoint))
        .route("/users/:user_id/sport/stats-weekly", get(get_weekly_sport_stats_endpoint))
        .route("/users/:user_id/sport/stats-monthly", get(get_monthly_sport_stats_endpoint))
        .route("/users/:user_id/sport/chart-data", get(get_sport_chart_data_endpoint))
        
        // ── BREATHING GET routes ───────────────────────────────────────────
        .route("/users/:user_id/breathing-sessions/latest", get(get_latest_breathing_session_endpoint))
        .route("/users/:user_id/breathing-sessions/today", get(get_today_breathing_sessions_endpoint))
        .route("/users/:user_id/breathing-sessions/period", get(get_breathing_sessions_by_period_endpoint))
        .route("/users/:user_id/breathing/duration-by-period", get(get_total_breathing_duration_by_period_endpoint))
        .route("/users/:user_id/breathing/count-by-period", get(get_breathing_session_count_by_period_endpoint))
        .route("/users/:user_id/breathing/average-frequency", get(get_average_breathing_usage_frequency_endpoint))
        
        // ── ALCOHOL GET routes ─────────────────────────────────────────────
        .route("/users/:user_id/alcohol/latest", get(get_latest_alcohol_entry_endpoint))
        .route("/users/:user_id/alcohol/entries-by-period", get(get_alcohol_entries_by_period_endpoint))
        .route("/users/:user_id/alcohol/blood-alcohol-level", get(get_current_blood_alcohol_level_endpoint))
        .route("/users/:user_id/alcohol/time-until-sobriety", get(get_time_until_sobriety_endpoint))
        
        .layer(middleware::from_fn_with_state(protected_state, validate_jwt))
        .with_state(pool)
}
