// Modèles de données et structures de requête/réponse
//
// Description (FR):
// Ce fichier définit toutes les structures (Request/Response) utilisées
// par l'API ainsi que les mapping vers les lignes SQL via `FromRow`.
// Il contient les types sérialisables envoyés au client et les types
// désérialisables reçus en payload JSON.

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Réponse d'erreur générique envoyée aux clients
#[derive(Debug, Serialize)]
pub struct ApiError {
    /// Message d'erreur lisible
    pub message: String,
}

// ── Authentification ─────────────────────────────────────────────────────────

/// Requête d'inscription: email + passphrase
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub passphrase: String,
}

/// Requête de connexion: email + passphrase
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub passphrase: String,
}

/// Réponse après authentification réussie
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,      // JWT token valide 7 jours
    pub user_id: i32,       // ID utilisateur
    pub email: String,      // Email confirmé
    pub encryption_key: Option<String>, // Clé AES-256 retournée au client (jamais stockée)
    pub encryption_salt: Option<String>, // Salt hex persisté en base
}

/// Données utilisateur récupérées de la base
#[derive(Debug, Serialize, FromRow)]
pub struct User {
    pub usrid: i32,                 // ID utilisateur
    pub email: String,              // Email de connexion
    pub passphrase_hash: String,    // Hash Argon2 (jamais la passphrase en clair)
    pub usrcreated_at: NaiveDateTime, // Timestamp création compte
    pub encryption_salt: Option<String>, // Salt hex pour dérivation clé AES-256
}

/// Clés de chiffrement dérivées pour le client
#[derive(Debug, Serialize)]
pub struct DerivedKeysResponse {
    pub encryption_key: String,  // Clé AES-256 encodée hex (32 bytes)
    pub salt: String,             // Salt aléatoire encodé hex (16 bytes)
}

// ── JWT Claims ───────────────────────────────────────────────────────────────

/// Données contenues dans le JWT token
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,  // ID utilisateur (subject)
    pub email: String, // Email pour identification
    pub iat: i64,     // Timestamp création (issued at)
    pub exp: i64,     // Timestamp expiration (expiration)
}

/// Statut de santé du serveur
#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,      // "healthy" ou "unhealthy"
    pub version: String,     // Version API
    pub timestamp: String,   // ISO 8601 timestamp
}

#[derive(Debug, Serialize)]
pub struct ApiInfo {
    pub name: String,
    pub description: String,
    pub version: String,
    pub endpoints: Vec<EndpointInfo>,
}

#[derive(Debug, Serialize)]
pub struct EndpointInfo {
    pub method: String,
    pub path: String,
    pub description: String,
}

#[derive(Debug, Serialize)]
pub struct CreatedResponse {
    pub id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub public_id: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub name: String,
    pub balance: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateFinanceTypeRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTransactionRequest {
    pub account_id: i32,
    pub type_id: i32,
    pub amount: f64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct CreatePlannedExpenseRequest {
    pub description: String,
    pub amount: f64,
    pub account_id: Option<i32>,
    pub type_id: Option<i32>,
    pub periodicite: String,
    pub intervalle: i32,
    pub next_date: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateHabitCategoryRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateHabitRequest {
    pub category_id: i32,
    pub title: String,
    pub description: String,
    pub habit_type: String,
}

#[derive(Debug, Deserialize)]
pub struct CompleteHabitRequest {
    pub date: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateSobrietyPeriodRequest {
    pub start_date: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateMoodTypeRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LogMoodRequest {
    pub type_id: i32,
    pub date: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LogHydrationRequest {
    pub date: String,
    pub quantity: i32,
    pub hydration_type: String,
    pub objective: i32,
}

#[derive(Debug, Deserialize)]
pub struct LogSleepRequest {
    pub date: String,
    pub time: String,
    pub duration: i32,
    pub quality: f64,
    pub is_restful: bool,
}

#[derive(Debug, Deserialize)]
pub struct LogMealRequest {
    pub date: String,
    pub time: String,
    pub name: String,
    pub calories: f64,
    pub proteins: f64,
    pub carbs: f64,
    pub fats: f64,
}

#[derive(Debug, Deserialize)]
pub struct LogBodyMeasurementRequest {
    pub date: String,
    pub weight: f64,
    pub height: f64,
    pub chest: f64,
    pub waist: f64,
    pub hips: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateSportTypeRequest {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct LogSportSessionRequest {
    pub type_id: i32,
    pub date: String,
    pub time: String,
    pub duration: i32,
    pub calories: f64,
    pub intensity: String,
}

#[derive(Debug, Deserialize)]
pub struct LogBreathingSessionRequest {
    pub date: String,
    pub time: String,
    pub duration: i32,
    pub frequency: String,
}

#[derive(Debug, Deserialize)]
pub struct LogAlcoholConsumptionRequest {
    pub date: String,
    pub time: String,
    pub alcohol_type: String,
    pub quantity: f64,
    pub percentage: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<String>,
}

// ── Zero-Knowledge ──────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EncryptedPayloadRequest {
    pub date: NaiveDate,
    pub iv: String,
    pub ciphertext: String,
    pub version: i32,
}

/// Ligne retournée depuis la table DONNEE_CHIFFREE.
/// Le serveur ne connaît jamais le contenu en clair.
#[derive(Debug, Serialize, FromRow)]
pub struct EncryptedEntry {
    pub dcid: i32,
    pub usrid: i32,
    pub dcdate: NaiveDate,
    pub dciv: String,
    pub dcciphertext: String,
    pub dcversion: i32,
}

// ── Profil & Dashboards ─────────────────────────────────────────────────────

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
    pub comid: Option<i32>,
    pub typid: Option<i32>,
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
    pub comid: Option<i32>,
    pub mouid: i32,
    pub facdate: NaiveDate,
    pub facperiodicite: String,
    pub facintervalle: i32,
    pub facdateprochain: Option<NaiveDate>,
    pub facdone: Option<i32>,
    pub moumontant: f64,
    pub moudescription: Option<String>,
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
    pub catid: Option<i32>,
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

#[derive(Debug, Serialize, FromRow)]
pub struct MoodType {
    pub humid: i32,
    pub humnom: String,
    pub humcolor: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct MoodEntry {
    pub usrid: i32,
    pub humid: Option<i32>,
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

#[derive(Debug, Serialize, FromRow)]
pub struct HydrationEntry {
    pub hydid: i32,
    pub usrid: i32,
    pub hyddate: NaiveDate,
    pub hydquantite: i32,
    pub hydobjectif: i32,
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

#[derive(Debug, Serialize, FromRow)]
pub struct SleepEntry {
    pub somid: i32,
    pub usrid: i32,
    pub somdate: NaiveDate,
    pub somcoucher: NaiveTime,
    pub somlever: NaiveTime,
    pub somduree: Option<i32>,
    pub somreposant: Option<i32>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SleepDuration {
    pub sleep_duration_minutes: i32,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AverageBedtime {
    pub average_bedtime: Option<NaiveTime>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AverageSleep {
    pub weekly_sleep_average: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Meal {
    pub repid: i32,
    pub usrid: i32,
    pub repdate: NaiveDate,
    pub repdescription: Option<String>,
    pub repcalories: Option<f64>,
    pub repproteines: Option<f64>,
    pub repglucides: Option<f64>,
    pub replipides: Option<f64>,
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
pub struct CarbTotal {
    pub carb_total: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct FatTotal {
    pub fat_total: f64,
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

#[derive(Debug, Serialize, FromRow)]
pub struct BodyMeasurement {
    pub mesid: i32,
    pub usrid: i32,
    pub mesdate: NaiveDate,
    pub mespoids: f64,
    pub mestaille: Option<f64>,
    #[sqlx(rename = "mesIMC")]
    pub mes_imc: Option<f64>,
    #[sqlx(rename = "mesMetaBasal")]
    pub mes_meta_basal: Option<f64>,
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
    #[sqlx(rename = "mesIMC")]
    pub mes_imc: Option<f64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct HealthMetrics {
    pub latest_weight: f64,
    pub latest_height: Option<f64>,
    pub latest_bmi: Option<f64>,
    pub latest_basal_metabolism: Option<f64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SportType {
    pub stypid: i32,
    pub stypnom: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct SportSession {
    pub seaid: i32,
    pub usrid: i32,
    pub stypid: Option<i32>,
    pub seadate: NaiveDate,
    pub seaduree: i32,
    pub seacalories: Option<f64>,
    pub seaintensite: String,
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

#[derive(Debug, Serialize, FromRow)]
pub struct AlcoholEntry {
    pub alcid: i32,
    pub usrid: i32,
    pub alcdateheure: NaiveDateTime,
    pub alcalcoolemie: Option<f64>,
    pub alctempsobre: Option<f64>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct BloodAlcoholLevel {
    pub alcalcoolemie: f64,
}

#[derive(Debug, Serialize, FromRow)]
pub struct TimeUntilSobriety {
    pub alctempsobre: Option<f64>,
}
