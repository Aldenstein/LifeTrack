use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use serde::Serialize;
use sqlx::FromRow;

// ===== GLOBAL MODELS =====
#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserProfile {
    pub usrid: i32,
    pub usrpublic_id: String,
    pub usrcreated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TodayDashboard {
    pub today_water_total: f64,
    pub today_calories_total: f64,
    pub today_protein_total: f64,
    pub today_carb_total: f64,
    pub today_fat_total: f64,
    pub today_sport_duration: i64,
    pub today_sport_count: i64,
    pub open_todos: i64,
    pub today_bilan_count: i64,
    pub today_mood_count: i64,
    pub today_sleep_count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ModuleDataPeriod {
    pub repid: Option<i32>,
    pub usrid: Option<i32>,
    pub repdate: Option<NaiveDate>,
    pub repcalories: Option<f64>,
    pub repproteines: Option<f64>,
    pub repglucides: Option<f64>,
    pub replipides: Option<f64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct LatestModuleValues {
    pub module_name: String,
    pub metric_name: String,
    pub metric_value: String,
    pub metric_date: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ActiveAlert {
    pub alert_type: String,
    pub item_id: i32,
    pub due_date: Option<NaiveDate>,
}

// ===== FINANCES MODELS =====
#[derive(Debug, Serialize, FromRow)]
pub struct Account {
    pub comid: i32,
    pub usrid: i32,
    pub comnom: String,
    pub comsolde: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AccountBalance {
    pub comid: i32,
    pub comnom: String,
    pub comsolde: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct FinanceType {
    pub typid: i32,
    pub typtitre: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Transaction {
    pub mouid: i32,
    pub usrid: i32,
    pub comid: i32,
    pub typid: i32,
    pub moudate: NaiveDate,
    pub moumontant: f64,
    pub moudescription: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct IncomeTotal {
    pub income_total: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ExpenseTotal {
    pub expense_total: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct NetBalance {
    pub net_balance: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct PlannedExpense {
    pub facid: i32,
    pub usrid: i32,
    pub comid: i32,
    pub facnom: String,
    pub facmontant: f64,
    pub facdateprochain: Option<NaiveDate>,
    pub facdone: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TopExpenseType {
    pub typid: i32,
    pub typtitre: String,
    pub total_expense: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct BalanceHistory {
    pub moudate: NaiveDate,
    pub running_balance: f64,
}

// ===== HABITS MODELS =====
#[derive(Debug, Serialize, FromRow)]
pub struct HabitCategory {
    pub catid: i32,
    pub catnom: String,
    pub catplus: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Habit {
    pub habid: i32,
    pub usrid: i32,
    pub catid: i32,
    pub habnom: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TodayHabit {
    pub habid: i32,
    pub habnom: String,
    pub catnom: Option<String>,
    pub catplus: Option<String>,
    pub done_today: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CompletedHabit {
    pub habid: i32,
    pub habnom: String,
    pub catnom: String,
    pub catplus: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HabitSummary {
    pub total_habits: i64,
    pub completed_habits: i64,
    pub pending_habits: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HabitScore {
    pub habit_score: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct WeeklyHabitScore {
    pub bildate: NaiveDate,
    pub habit_score: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HabitHistory {
    pub bildate: NaiveDate,
    pub hbdone: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HabitCompletionRate {
    pub completion_rate: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HabitConsistency {
    pub habid: i32,
    pub habnom: String,
    pub consistency_rate: f64,
}

// ===== SOBRIETY MODELS =====
#[derive(Debug, Serialize, FromRow)]
pub struct SobrietyPeriod {
    pub sobid: i32,
    pub usrid: i32,
    pub sobdebut: NaiveDateTime,
    pub sobfin: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SobrietyDuration {
    pub sobriety_hours: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TotalSobrietyDuration {
    pub total_sobriety_hours: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SobrietyStats {
    pub sobriety_period_count: i64,
    pub sobriety_hours: i64,
}

// ===== MOOD MODELS =====
#[derive(Debug, Serialize, FromRow)]
pub struct MoodType {
    pub humid: i32,
    pub humnom: String,
    pub humcolor: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MoodEntry {
    pub dhid: i32,
    pub usrid: i32,
    pub humid: i32,
    pub dhdate: NaiveDate,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MostFrequentMood {
    pub humid: i32,
    pub humnom: String,
    pub mood_count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MoodDistribution {
    pub humid: i32,
    pub humnom: String,
    pub mood_count: i64,
}

// ===== HYDRATION MODELS =====
#[derive(Debug, Serialize, FromRow)]
pub struct HydrationEntry {
    pub hydid: i32,
    pub usrid: i32,
    pub hyddate: NaiveDate,
    pub hydquantite: f64,
    pub hydobjectif: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct WaterTotal {
    pub water_total: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HydrationGoalProgress {
    pub goal_progress_percent: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AverageWater {
    pub weekly_average: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HydrationGoalHistory {
    pub hyddate: NaiveDate,
    pub hydobjectif: f64,
}

// ===== SLEEP MODELS =====
#[derive(Debug, Serialize, FromRow)]
pub struct SleepEntry {
    pub somid: i32,
    pub usrid: i32,
    pub somdate: NaiveDate,
    pub somcoucher: NaiveTime,
    pub somlever: NaiveTime,
    pub somduree: Option<i32>,
    pub somreposant: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SleepDuration {
    pub sleep_duration_minutes: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AverageBedtime {
    pub average_bedtime: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AverageSleep {
    pub weekly_sleep_average: f64,
}

// ===== NUTRITION MODELS =====
#[derive(Debug, Serialize, FromRow)]
pub struct Meal {
    pub repid: i32,
    pub usrid: i32,
    pub repdate: NaiveDate,
    pub repcalories: f64,
    pub repproteines: f64,
    pub repglucides: f64,
    pub replipides: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CalorieTotal {
    pub calorie_total: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct ProteinTotal {
    pub protein_total: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MacroDistribution {
    pub proteins: f64,
    pub carbs: f64,
    pub fats: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct NutritionHistory {
    pub repdate: NaiveDate,
    pub calories: f64,
    pub proteins: f64,
    pub carbs: f64,
    pub fats: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AverageCalories {
    pub weekly_calorie_average: f64,
}

// ===== BODY MEASUREMENT MODELS =====
#[derive(Debug, Serialize, FromRow)]
pub struct BodyMeasurement {
    pub mesid: i32,
    pub usrid: i32,
    pub mesdate: NaiveDate,
    pub mespoids: f64,
    pub mestaille: Option<f64>,
    pub mesIMC: Option<f64>,
    pub mesMetaBasal: Option<f64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct WeightEntry {
    pub mesdate: NaiveDate,
    pub mespoids: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct WeightProgress {
    pub first_weight: f64,
    pub last_weight: f64,
    pub weight_delta: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct CurrentBmi {
    pub mesIMC: Option<f64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HealthMetrics {
    pub latest_weight: f64,
    pub latest_height: Option<f64>,
    pub latest_bmi: Option<f64>,
    pub latest_basal_metabolism: Option<f64>,
}

// ===== SPORT MODELS =====
#[derive(Debug, Serialize, FromRow)]
pub struct SportType {
    pub stypid: i32,
    pub stypnom: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SportSession {
    pub seaid: i32,
    pub usrid: i32,
    pub stypid: i32,
    pub seadate: NaiveDate,
    pub seaduree: i32,
    pub seacalories: f64,
    pub seaintensite: Option<i32>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SportDurationTotal {
    pub sport_duration_total: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SportSessionCount {
    pub sport_session_count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct BurnedCalories {
    pub burned_calories: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MostPracticedSport {
    pub stypid: Option<i32>,
    pub stypnom: Option<String>,
    pub session_count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SportStats {
    pub session_count: i64,
    pub total_duration: i64,
    pub total_calories: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SportChartData {
    pub seadate: NaiveDate,
    pub duration: i64,
    pub calories: f64,
}

// ===== BREATHING/COHERENCE MODELS =====
#[derive(Debug, Serialize, FromRow)]
pub struct BreathingSession {
    pub cohid: i32,
    pub usrid: i32,
    pub cohdateheure: NaiveDateTime,
    pub cohduree: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct BreathingDurationTotal {
    pub breathing_duration_total: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct BreathingSessionCount {
    pub breathing_session_count: i64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct BreathingFrequency {
    pub average_frequency: f64,
}

// ===== ALCOHOL MODELS =====
#[derive(Debug, Serialize, FromRow)]
pub struct AlcoholEntry {
    pub alcid: i32,
    pub usrid: i32,
    pub alcdateheure: NaiveDateTime,
    pub alcalcoolemie: f64,
    pub alctempsobre: Option<i32>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct BloodAlcoholLevel {
    pub alcalcoolemie: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TimeUntilSobriety {
    pub alctempsobre: Option<i32>,
}