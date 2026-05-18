// GET endpoints for all read-only database functions
// Organized by domain: Finance, Habits, Sobriety, Mood, Hydration, Sleep, Meals, Body, Sport, Breathing, Alcohol

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDate;
use serde::Deserialize;

use crate::db::*;
use crate::models::{ApiError, CarbTotal, FatTotal};
use crate::utils::parse_date;

// Helper for general GET errors
fn get_error(error: sqlx::Error) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            message: format!("Database error: {error}"),
        }),
    )
}

fn not_found() -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiError {
            message: "Resource not found".to_string(),
        }),
    )
}

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
) -> Result<Json<Vec<Account>>, (StatusCode, Json<ApiError>)> {
    get_user_accounts(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_account_balances_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<AccountBalance>>, (StatusCode, Json<ApiError>)> {
    get_account_balances(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_finance_types_endpoint(
    State(pool): State<crate::db::DbPool>,
) -> Result<Json<Vec<FinanceType>>, (StatusCode, Json<ApiError>)> {
    get_finance_types(&pool).await.map(Json).map_err(get_error)
}

pub async fn get_transactions_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_transactions_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_transactions_by_account_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, account_id)): Path<(i32, i32)>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, Json<ApiError>)> {
    get_transactions_by_account(&pool, user_id, account_id)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_transactions_by_type_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, type_id)): Path<(i32, i32)>,
) -> Result<Json<Vec<Transaction>>, (StatusCode, Json<ApiError>)> {
    get_transactions_by_type(&pool, user_id, type_id)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_income_total_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<IncomeSummary>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_income_total_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_expense_total_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<ExpenseSummary>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_expense_total_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_net_balance_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<BalanceSummary>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_net_balance_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_planned_expenses_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<PlannedExpense>>, (StatusCode, Json<ApiError>)> {
    get_planned_expenses(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_upcoming_planned_expenses_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<PlannedExpense>>, (StatusCode, Json<ApiError>)> {
    get_upcoming_planned_expenses(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_top_expense_types_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<TopExpensesQuery>,
) -> Result<Json<Vec<ExpenseTypeRanking>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_top_expense_types(&pool, user_id, start, end, params.limit)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_balance_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<BalanceHistory>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_balance_history(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

// ── HABIT endpoints ────────────────────────────────────────────

pub async fn get_habit_categories_endpoint(
    State(pool): State<crate::db::DbPool>,
) -> Result<Json<Vec<HabitCategory>>, (StatusCode, Json<ApiError>)> {
    get_habit_categories(&pool).await.map(Json).map_err(get_error)
}

pub async fn get_active_habits_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Habit>>, (StatusCode, Json<ApiError>)> {
    get_active_habits(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_positive_habits_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Habit>>, (StatusCode, Json<ApiError>)> {
    get_positive_habits(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_negative_habits_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Habit>>, (StatusCode, Json<ApiError>)> {
    get_negative_habits(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_today_habits_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<TodayHabit>>, (StatusCode, Json<ApiError>)> {
    get_today_habits(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_completed_habits_today_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<CompletedHabit>>, (StatusCode, Json<ApiError>)> {
    get_completed_habits_today(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_pending_habits_today_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<CompletedHabit>>, (StatusCode, Json<ApiError>)> {
    get_pending_habits_today(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_today_habit_summary_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<HabitSummary>, (StatusCode, Json<ApiError>)> {
    get_today_habit_summary(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_today_habit_score_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<HabitScore>, (StatusCode, Json<ApiError>)> {
    get_today_habit_score(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_weekly_habit_score_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<WeeklyHabitScore>>, (StatusCode, Json<ApiError>)> {
    get_weekly_habit_score(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_habit_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, habit_id)): Path<(i32, i32)>,
) -> Result<Json<Vec<HabitHistory>>, (StatusCode, Json<ApiError>)> {
    get_habit_history(&pool, user_id, habit_id).await.map(Json).map_err(get_error)
}

pub async fn get_habit_completion_rate_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, habit_id)): Path<(i32, i32)>,
) -> Result<Json<HabitCompletionRate>, (StatusCode, Json<ApiError>)> {
    get_habit_completion_rate(&pool, user_id, habit_id)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_most_consistent_habits_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<LimitQuery>,
) -> Result<Json<Vec<HabitConsistency>>, (StatusCode, Json<ApiError>)> {
    get_most_consistent_habits(&pool, user_id, params.limit)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_least_consistent_habits_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<LimitQuery>,
) -> Result<Json<Vec<HabitConsistency>>, (StatusCode, Json<ApiError>)> {
    get_least_consistent_habits(&pool, user_id, params.limit)
        .await
        .map(Json)
        .map_err(get_error)
}

// ── SOBRIETY endpoints ─────────────────────────────────────────

pub async fn get_current_sobriety_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<SobrietyPeriod>>, (StatusCode, Json<ApiError>)> {
    get_current_sobriety_period(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_current_sobriety_duration_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<SobrietyDuration>>, (StatusCode, Json<ApiError>)> {
    get_current_sobriety_duration(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_sobriety_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<SobrietyPeriod>>, (StatusCode, Json<ApiError>)> {
    get_sobriety_history(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_total_sobriety_duration_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<TotalSobrietyDuration>, (StatusCode, Json<ApiError>)> {
    get_total_sobriety_duration(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_sobriety_stats_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<DaysQuery>,
) -> Result<Json<SobrietyStats>, (StatusCode, Json<ApiError>)> {
    get_sobriety_stats_by_period(&pool, user_id, params.days)
        .await
        .map(Json)
        .map_err(get_error)
}

// ── MOOD endpoints ─────────────────────────────────────────────

pub async fn get_mood_types_endpoint(
    State(pool): State<crate::db::DbPool>,
) -> Result<Json<Vec<MoodType>>, (StatusCode, Json<ApiError>)> {
    get_mood_types(&pool).await.map(Json).map_err(get_error)
}

pub async fn get_today_mood_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<MoodEntry>>, (StatusCode, Json<ApiError>)> {
    get_today_mood(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_mood_by_date_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<DateQuery>,
) -> Result<Json<Option<MoodEntry>>, (StatusCode, Json<ApiError>)> {
    let date = parse_date(&params.date)?;
    get_mood_by_date(&pool, user_id, date).await.map(Json).map_err(get_error)
}

#[derive(Deserialize)]
pub struct DateQuery {
    date: String,
}

pub async fn get_monthly_moods_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<MonthQuery>,
) -> Result<Json<Vec<MoodEntry>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_monthly_moods(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

#[derive(Deserialize)]
pub struct MonthQuery {
    start: String,
    end: String,
}

pub async fn get_most_frequent_mood_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<MostFrequentMood>>, (StatusCode, Json<ApiError>)> {
    get_most_frequent_mood(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_mood_distribution_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<MoodDistribution>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_mood_distribution_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

// ── HYDRATION endpoints ────────────────────────────────────────

pub async fn get_today_hydration_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<HydrationEntry>>, (StatusCode, Json<ApiError>)> {
    get_today_hydration(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_today_hydration_goal_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<HydrationEntry>>, (StatusCode, Json<ApiError>)> {
    get_today_hydration_goal(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_hydration_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<HydrationEntry>>, (StatusCode, Json<ApiError>)> {
    get_hydration_history(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_today_water_total_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<WaterTotal>, (StatusCode, Json<ApiError>)> {
    get_today_water_total(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_hydration_goal_progress_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<HydrationGoalProgress>, (StatusCode, Json<ApiError>)> {
    get_hydration_goal_progress(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_hydration_goal_reached_days_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<HydrationGoalHistory>>, (StatusCode, Json<ApiError>)> {
    get_hydration_goal_reached_days(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_hydration_goal_missed_days_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<HydrationGoalHistory>>, (StatusCode, Json<ApiError>)> {
    get_hydration_goal_missed_days(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_weekly_hydration_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageWater>, (StatusCode, Json<ApiError>)> {
    get_weekly_hydration_average(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_monthly_hydration_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageWater>, (StatusCode, Json<ApiError>)> {
    get_monthly_hydration_average(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_hydration_goal_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<HydrationGoalHistory>>, (StatusCode, Json<ApiError>)> {
    get_hydration_goal_history(&pool, user_id).await.map(Json).map_err(get_error)
}

// ── SLEEP endpoints ────────────────────────────────────────────

pub async fn get_latest_sleep_entry_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<SleepEntry>>, (StatusCode, Json<ApiError>)> {
    get_latest_sleep_entry(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_today_sleep_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<SleepEntry>>, (StatusCode, Json<ApiError>)> {
    get_today_sleep(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_sleep_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<SleepEntry>>, (StatusCode, Json<ApiError>)> {
    get_sleep_history(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_restful_sleep_entries_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<SleepEntry>>, (StatusCode, Json<ApiError>)> {
    get_restful_sleep_entries(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_non_restful_sleep_entries_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<SleepEntry>>, (StatusCode, Json<ApiError>)> {
    get_non_restful_sleep_entries(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_weekly_sleep_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageSleep>, (StatusCode, Json<ApiError>)> {
    get_weekly_sleep_average(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_monthly_sleep_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageSleep>, (StatusCode, Json<ApiError>)> {
    get_monthly_sleep_average(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_short_sleep_entries_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, min_duration)): Path<(i32, i32)>,
) -> Result<Json<Vec<SleepEntry>>, (StatusCode, Json<ApiError>)> {
    get_short_sleep_entries(&pool, user_id, min_duration).await.map(Json).map_err(get_error)
}

// ── MEAL endpoints ─────────────────────────────────────────────

pub async fn get_today_meals_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Meal>>, (StatusCode, Json<ApiError>)> {
    get_today_meals(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_meals_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<Meal>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_meals_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_latest_meal_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<Meal>>, (StatusCode, Json<ApiError>)> {
    get_latest_meal(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_today_calorie_total_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<CalorieTotal>, (StatusCode, Json<ApiError>)> {
    get_today_calorie_total(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_today_protein_total_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<ProteinTotal>, (StatusCode, Json<ApiError>)> {
    get_today_protein_total(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_today_carb_total_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<CarbTotal>, (StatusCode, Json<ApiError>)> {
    get_today_carb_total(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_today_fat_total_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<FatTotal>, (StatusCode, Json<ApiError>)> {
    get_today_fat_total(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_daily_macro_distribution_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<MacroDistribution>, (StatusCode, Json<ApiError>)> {
    get_daily_macro_distribution(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_nutrition_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<NutritionEntry>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_nutrition_history(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_weekly_calorie_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageCalories>, (StatusCode, Json<ApiError>)> {
    get_weekly_calorie_average(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_monthly_calorie_average_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<AverageCalories>, (StatusCode, Json<ApiError>)> {
    get_monthly_calorie_average(&pool, user_id).await.map(Json).map_err(get_error)
}

// ── BODY MEASUREMENT endpoints ────────────────────────────────

pub async fn get_latest_body_measurement_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<BodyMeasurement>>, (StatusCode, Json<ApiError>)> {
    get_latest_body_measurement(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_weight_history_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<WeightEntry>>, (StatusCode, Json<ApiError>)> {
    get_weight_history(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_weight_chart_data_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<WeightChartData>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_weight_chart_data(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_weight_progress_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<WeightProgress>>, (StatusCode, Json<ApiError>)> {
    get_weight_progress(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_current_bmi_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<CurrentBmi>>, (StatusCode, Json<ApiError>)> {
    get_current_bmi(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_health_derived_metrics_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<HealthMetrics>>, (StatusCode, Json<ApiError>)> {
    get_health_derived_metrics(&pool, user_id).await.map(Json).map_err(get_error)
}

// ── SPORT endpoints ────────────────────────────────────────────

pub async fn get_sport_types_endpoint(
    State(pool): State<crate::db::DbPool>,
) -> Result<Json<Vec<SportType>>, (StatusCode, Json<ApiError>)> {
    get_sport_types(&pool).await.map(Json).map_err(get_error)
}

pub async fn get_today_sport_sessions_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<SportSession>>, (StatusCode, Json<ApiError>)> {
    get_today_sport_sessions(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_sport_sessions_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<SportSession>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_sport_sessions_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_latest_sport_session_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<SportSession>>, (StatusCode, Json<ApiError>)> {
    get_latest_sport_session(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_total_sport_duration_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<SportDurationStats>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_total_sport_duration_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_sport_session_count_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<SportCountStats>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_sport_session_count_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_burned_calories_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<SportCaloriesStats>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_burned_calories_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_sport_sessions_by_type_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path((user_id, sport_type_id)): Path<(i32, i32)>,
) -> Result<Json<Vec<SportSession>>, (StatusCode, Json<ApiError>)> {
    get_sport_sessions_by_type(&pool, user_id, sport_type_id)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_most_practiced_sport_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<MostPracticedSport>>, (StatusCode, Json<ApiError>)> {
    get_most_practiced_sport(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_weekly_sport_stats_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<SportStats>, (StatusCode, Json<ApiError>)> {
    get_weekly_sport_stats(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_monthly_sport_stats_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<SportStats>, (StatusCode, Json<ApiError>)> {
    get_monthly_sport_stats(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_sport_chart_data_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<SportChartData>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_sport_chart_data(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

// ── BREATHING endpoints ────────────────────────────────────────

pub async fn get_latest_breathing_session_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<BreathingSession>>, (StatusCode, Json<ApiError>)> {
    get_latest_breathing_session(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_today_breathing_sessions_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<BreathingSession>>, (StatusCode, Json<ApiError>)> {
    get_today_breathing_sessions(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_breathing_sessions_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<BreathingSession>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_breathing_sessions_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_total_breathing_duration_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<BreathingDurationStats>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_total_breathing_duration_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_breathing_session_count_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<BreathingCountStats>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_breathing_session_count_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_average_breathing_usage_frequency_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<BreathingFrequency>, (StatusCode, Json<ApiError>)> {
    get_average_breathing_usage_frequency(&pool, user_id)
        .await
        .map(Json)
        .map_err(get_error)
}

// ── ALCOHOL endpoints ──────────────────────────────────────────

pub async fn get_latest_alcohol_entry_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<AlcoholEntry>>, (StatusCode, Json<ApiError>)> {
    get_latest_alcohol_entry(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_alcohol_entries_by_period_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<PeriodQuery>,
) -> Result<Json<Vec<AlcoholEntry>>, (StatusCode, Json<ApiError>)> {
    let start = parse_date(&params.start)?;
    let end = parse_date(&params.end)?;
    get_alcohol_entries_by_period(&pool, user_id, start, end)
        .await
        .map(Json)
        .map_err(get_error)
}

pub async fn get_current_blood_alcohol_level_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<BloodAlcoholLevel>>, (StatusCode, Json<ApiError>)> {
    get_current_blood_alcohol_level(&pool, user_id).await.map(Json).map_err(get_error)
}

pub async fn get_time_until_sobriety_endpoint(
    State(pool): State<crate::db::DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Option<TimeUntilSobriety>>, (StatusCode, Json<ApiError>)> {
    get_time_until_sobriety(&pool, user_id).await.map(Json).map_err(get_error)
}
