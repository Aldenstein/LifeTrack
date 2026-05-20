// GET endpoints for all read-only database functions
// Organized by domain: Finance, Habits, Sobriety, Mood, Hydration, Sleep, Meals, Body, Sport, Breathing, Alcohol
//
// Description (FR):
// Ce fichier expose les handlers HTTP en lecture seule pour l'API.
// Chaque fonction fait l'intermédiaire entre la requête HTTP (extraction
// des paramètres, parsing des dates) et les fonctions de la couche
// `db` qui effectuent les requêtes SQL. Utilise le type d'erreur centralisé
// `AppError` pour un traitement cohérent des erreurs.
//
// Conventions:
// - Les structures `Query` servent à désérialiser les paramètres GET.
// - Les endpoints convertissent souvent des chaînes de dates via
//   `utils::parse_date` avant d'appeler les fonctions `db`.
// - Les endpoints retournent `Result<Json<T>>` via le type `AppError`.

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::db::*;
use crate::models::*;
use crate::errors::{AppError, Result};
use crate::utils::parse_date;

// ── Query param structs ────────────────────────────────────────

#[derive(Deserialize)]
pub struct PeriodQuery {
    pub start: String,
    pub end: String,
}

#[derive(Deserialize)]
pub struct TopExpensesQuery {
    pub start: String,
    pub end: String,
    #[serde(default = "default_limit")]
    pub limit: i32,
}

fn default_limit() -> i32 { 10 }

#[derive(Deserialize)]
pub struct LimitQuery {
    #[serde(default = "default_limit")]
    pub limit: i32,
}

#[derive(Deserialize)]
pub struct DaysQuery {
    #[serde(default = "default_days")]
    pub days: i32,
}

fn default_days() -> i32 { 30 }

// ── FINANCE endpoints ──────────────────────────────────────────

pub async fn get_user_accounts_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Account>>> {
    get_user_accounts(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_account_balances_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<AccountBalance>>> {
    get_account_balances(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_finance_types_endpoint(
    State(pool): State<crate::db::DbPool>,
) -> Result<Json<Vec<FinanceType>>> {
    get_finance_types(&pool).await.map(Json).map_err(Into::into)
}

pub async fn get_transactions_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<Transaction>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_transactions_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_transactions_by_account_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, account_id)): Path<(i32, i32)>,
) -> Result<Json<Vec<Transaction>>> {
    get_transactions_by_account(&pool, user_id, account_id)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_transactions_by_type_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, type_id)): Path<(i32, i32)>,
) -> Result<Json<Vec<Transaction>>> {
    get_transactions_by_type(&pool, user_id, type_id)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_income_total_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<IncomeTotal>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_income_total_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_expense_total_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<ExpenseTotal>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_expense_total_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_net_balance_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<NetBalance>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_net_balance_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_planned_expenses_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<PlannedExpense>>> {
    get_planned_expenses(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_upcoming_planned_expenses_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<PlannedExpense>>> {
    get_upcoming_planned_expenses(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_top_expense_types_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<TopExpensesQuery>,
) -> Result<Json<Vec<TopExpenseType>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_top_expense_types(&pool, user_id, start, end, params.limit)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_balance_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<BalanceHistory>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_balance_history(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

// ── HABIT endpoints ────────────────────────────────────────────

pub async fn get_habit_categories_endpoint(
    State(pool): State<crate::db::DbPool>,
) -> Result<Json<Vec<HabitCategory>>> {
    get_habit_categories(&pool).await.map(Json).map_err(Into::into)
}

pub async fn get_active_habits_endpoint(
    State(pool): State<crate::db::DbPool>, e'
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Habit>>> {
    get_active_habits(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_positive_habits_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Habit>>> {
    get_positive_habits(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_negative_habits_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Habit>>> {
    get_negative_habits(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_today_habits_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<TodayHabit>>> {
    get_today_habits(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_completed_habits_today_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<CompletedHabit>>> {
    get_completed_habits_today(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_pending_habits_today_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<CompletedHabit>>> {
    get_pending_habits_today(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_today_habit_summary_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<HabitSummary>> {
    get_today_habit_summary(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_today_habit_score_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<HabitScore>> {
    get_today_habit_score(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_weekly_habit_score_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<WeeklyHabitScore>>> {
    get_weekly_habit_score(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_habit_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, habit_id)): Path<(i32, i32)>,
) -> Result<Json<Vec<HabitHistory>>> {
    get_habit_history(&pool, user_id, habit_id).await.map(Json).map_err(Into::into)
}

pub async fn get_habit_completion_rate_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, habit_id)): Path<(i32, i32)>,
) -> Result<Json<HabitCompletionRate>> {
    get_habit_completion_rate(&pool, user_id, habit_id)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_most_consistent_habits_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<LimitQuery>,
) -> Result<Json<Vec<HabitConsistency>>> {
    get_most_consistent_habits(&pool, user_id, params.limit)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_least_consistent_habits_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<LimitQuery>,
) -> Result<Json<Vec<HabitConsistency>>> {
    get_least_consistent_habits(&pool, user_id, params.limit)
        .await
        .map(Json).map_err(Into::into)
        
}

// ── SOBRIETY endpoints ─────────────────────────────────────────

pub async fn get_current_sobriety_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<SobrietyPeriod>>> {
    get_current_sobriety_period(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_current_sobriety_duration_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<SobrietyDuration>>> {
    get_current_sobriety_duration(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_sobriety_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<SobrietyPeriod>>> {
    get_sobriety_history(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_total_sobriety_duration_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<TotalSobrietyDuration>> {
    get_total_sobriety_duration(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_sobriety_stats_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<DaysQuery>,
) -> Result<Json<SobrietyStats>> {
    get_sobriety_stats_by_period(&pool, user_id, params.days)
        .await
        .map(Json).map_err(Into::into)
        
}

// ── MOOD endpoints ─────────────────────────────────────────────

pub async fn get_mood_types_endpoint(
    State(pool): State<crate::db::DbPool>,
) -> Result<Json<Vec<MoodType>>> {
    get_mood_types(&pool).await.map(Json).map_err(Into::into)
}

pub async fn get_today_mood_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<MoodEntry>>> {
    get_today_mood(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_mood_by_date_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<DateQuery>,
) -> Result<Json<Option<MoodEntry>>> {
    let date = parse_date(&params.date)?;
    get_mood_by_date(&pool, user_id, date).await.map(Json).map_err(Into::into)
}

#[derive(Deserialize)]
pub struct DateQuery {
    date: String,
}

pub async fn get_monthly_moods_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<MonthQuery>,
) -> Result<Json<Vec<MoodEntry>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_monthly_moods(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

#[derive(Deserialize)]
pub struct MonthQuery {
    start: String,
    end: String,
}

pub async fn get_most_frequent_mood_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<MostFrequentMood>>> {
    get_most_frequent_mood(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_mood_distribution_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<MoodDistribution>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_mood_distribution_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

// ── HYDRATION endpoints ────────────────────────────────────────

pub async fn get_today_hydration_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<HydrationEntry>>> {
    get_today_hydration(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_today_hydration_goal_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<HydrationEntry>>> {
    get_today_hydration_goal(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_hydration_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<HydrationEntry>>> {
    get_hydration_history(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_today_water_total_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<WaterTotal>> {
    get_today_water_total(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_hydration_goal_progress_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<HydrationGoalProgress>> {
    get_hydration_goal_progress(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_hydration_goal_reached_days_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<HydrationGoalHistory>>> {
    get_hydration_goal_reached_days(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_hydration_goal_missed_days_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<HydrationGoalHistory>>> {
    get_hydration_goal_missed_days(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_weekly_hydration_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageWater>> {
    get_weekly_hydration_average(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_monthly_hydration_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageWater>> {
    get_monthly_hydration_average(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_hydration_goal_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<HydrationGoalHistory>>> {
    get_hydration_goal_history(&pool, user_id).await.map(Json).map_err(Into::into)
}

// ── SLEEP endpoints ────────────────────────────────────────────

pub async fn get_latest_sleep_entry_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<SleepEntry>>> {
    get_latest_sleep_entry(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_today_sleep_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<SleepEntry>>> {
    get_today_sleep(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_sleep_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<SleepEntry>>> {
    get_sleep_history(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_restful_sleep_entries_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<SleepEntry>>> {
    get_restful_sleep_entries(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_non_restful_sleep_entries_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<SleepEntry>>> {
    get_non_restful_sleep_entries(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_weekly_sleep_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageSleep>> {
    get_weekly_sleep_average(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_monthly_sleep_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageSleep>> {
    get_monthly_sleep_average(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_short_sleep_entries_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, min_duration)): Path<(i32, i32)>,
) -> Result<Json<Vec<SleepEntry>>> {
    get_short_sleep_entries(&pool, user_id, min_duration).await.map(Json).map_err(Into::into)
}

// ── MEAL endpoints ─────────────────────────────────────────────

pub async fn get_today_meals_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Meal>>> {
    get_today_meals(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_meals_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<Meal>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_meals_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_latest_meal_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<Meal>>> {
    get_latest_meal(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_today_calorie_total_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<CalorieTotal>> {
    get_today_calorie_total(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_today_protein_total_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<ProteinTotal>> {
    get_today_protein_total(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_today_carb_total_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<CarbTotal>> {
    get_today_carb_total(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_today_fat_total_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<FatTotal>> {
    get_today_fat_total(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_daily_macro_distribution_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<MacroDistribution>> {
    get_daily_macro_distribution(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_nutrition_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<NutritionHistory>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_nutrition_history(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_weekly_calorie_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageCalories>> {
    get_weekly_calorie_average(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_monthly_calorie_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageCalories>> {
    get_monthly_calorie_average(&pool, user_id).await.map(Json).map_err(Into::into)
}

// ── BODY MEASUREMENT endpoints ────────────────────────────────

pub async fn get_latest_body_measurement_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<BodyMeasurement>>> {
    get_latest_body_measurement(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_weight_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<WeightEntry>>> {
    get_weight_history(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_weight_chart_data_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<WeightEntry>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_weight_chart_data(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_weight_progress_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<WeightProgress>>> {
    get_weight_progress(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_current_bmi_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<CurrentBmi>>> {
    get_current_bmi(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_health_derived_metrics_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<HealthMetrics>>> {
    get_health_derived_metrics(&pool, user_id).await.map(Json).map_err(Into::into)
}

// ── SPORT endpoints ────────────────────────────────────────────

pub async fn get_sport_types_endpoint(
    State(pool): State<crate::db::DbPool>,
) -> Result<Json<Vec<SportType>>> {
    get_sport_types(&pool).await.map(Json).map_err(Into::into)
}

pub async fn get_today_sport_sessions_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<SportSession>>> {
    get_today_sport_sessions(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_sport_sessions_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<SportSession>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_sport_sessions_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_latest_sport_session_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<SportSession>>> {
    get_latest_sport_session(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_total_sport_duration_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<SportDurationTotal>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_total_sport_duration_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_sport_session_count_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<SportSessionCount>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_sport_session_count_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_burned_calories_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<BurnedCalories>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_burned_calories_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_sport_sessions_by_type_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, sport_type_id)): Path<(i32, i32)>,
) -> Result<Json<Vec<SportSession>>> {
    get_sport_sessions_by_type(&pool, user_id, sport_type_id)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_most_practiced_sport_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<MostPracticedSport>>> {
    get_most_practiced_sport(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_weekly_sport_stats_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<SportStats>> {
    get_weekly_sport_stats(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_monthly_sport_stats_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<SportStats>> {
    get_monthly_sport_stats(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_sport_chart_data_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<SportChartData>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_sport_chart_data(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

// ── BREATHING endpoints ────────────────────────────────────────

pub async fn get_latest_breathing_session_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<BreathingSession>>> {
    get_latest_breathing_session(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_today_breathing_sessions_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<BreathingSession>>> {
    get_today_breathing_sessions(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_breathing_sessions_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<BreathingSession>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_breathing_sessions_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_total_breathing_duration_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<BreathingDurationTotal>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_total_breathing_duration_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_breathing_session_count_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<BreathingSessionCount>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_breathing_session_count_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_average_breathing_usage_frequency_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<BreathingFrequency>> {
    get_average_breathing_usage_frequency(&pool, user_id)
        .await
        .map(Json).map_err(Into::into)
        
}

// ── ALCOHOL endpoints ──────────────────────────────────────────

pub async fn get_latest_alcohol_entry_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<AlcoholEntry>>> {
    get_latest_alcohol_entry(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_alcohol_entries_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<AlcoholEntry>>> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_alcohol_entries_by_period(&pool, user_id, start, end)
        .await
        .map(Json).map_err(Into::into)
        
}

pub async fn get_current_blood_alcohol_level_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<BloodAlcoholLevel>>> {
    get_current_blood_alcohol_level(&pool, user_id).await.map(Json).map_err(Into::into)
}

pub async fn get_time_until_sobriety_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<TimeUntilSobriety>>> {
    get_time_until_sobriety(&pool, user_id).await.map(Json).map_err(Into::into)
}


