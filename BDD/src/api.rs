// Module des endpoints HTTP
// Contient tous les handlers pour les requêtes REST
// Chaque fonction gère un endpoint spécifique

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{Local, NaiveDate};
use serde::Deserialize;

use crate::db::{
    create_account,
    create_finance_type,
    create_habit,
    create_habit_category,
    create_mood_type,
    create_planned_expense,
    create_sobriety_period,
    create_sport_type,
    create_todo,
    create_transaction,
    create_user,
    get_user_by_id,
    get_active_alerts_and_reminders,
    get_latest_module_values,
    get_today_dashboard,
    get_user_profile,
    log_alcohol_consumption,
    log_body_measurement,
    log_breathing_session,
    log_hydration,
    log_meal,
    log_mood,
    log_sleep,
    log_sport_session,
    mark_habit_complete,
    save_encrypted_entry,
    get_encrypted_entries,
    get_all_encrypted_entries,
    DbPool,
};
use crate::models::*;
use crate::utils::parse_date;

// Helpers pour les erreurs - simplifient les réponses d'erreur

/// Erreur lors du chargement du profil utilisateur
fn user_profile_error(error: sqlx::Error) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiError {
            message: format!("Unable to load user profile: {error}"),
        }),
    )
}

fn today_dashboard_error(error: sqlx::Error) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            message: format!("Unable to load today dashboard: {error}"),
        }),
    )
}

fn latest_module_values_error(error: sqlx::Error) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            message: format!("Unable to load latest module values: {error}"),
        }),
    )
}

fn alerts_reminders_error(error: sqlx::Error) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            message: format!("Unable to load alerts and reminders: {error}"),
        }),
    )
}

fn bad_request(message: impl Into<String>) -> (StatusCode, Json<ApiError>) {
    (StatusCode::BAD_REQUEST, Json(ApiError { message: message.into() }))
}

fn encrypted_error(error: sqlx::Error) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            message: format!("Encrypted storage error: {error}"),
        }),
    )
}

pub async fn user_profile(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserProfile>, (StatusCode, Json<ApiError>)> {
    get_user_profile(&pool, user_id).await.map(Json).map_err(user_profile_error)
}

pub async fn today_dashboard(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<TodayDashboard>, (StatusCode, Json<ApiError>)> {
    get_today_dashboard(&pool, user_id).await.map(Json).map_err(today_dashboard_error)
}

pub async fn latest_module_values(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<LatestModuleValues>>, (StatusCode, Json<ApiError>)> {
    get_latest_module_values(&pool, user_id).await.map(Json).map_err(latest_module_values_error)
}

pub async fn active_alerts_and_reminders(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<ActiveAlert>>, (StatusCode, Json<ApiError>)> {
    get_active_alerts_and_reminders(&pool, user_id).await.map(Json).map_err(alerts_reminders_error)
}

pub async fn health_check() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        timestamp: Local::now().to_rfc3339(),
    })
}

pub fn start_api() {
    println!("API started");
}

pub async fn create_user_endpoint(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let id = create_user(&pool, &payload.public_id)
        .await
        .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn create_account_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let id = create_account(&pool, user_id, &payload.name, payload.balance)
        .await
        .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn create_finance_type_endpoint(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateFinanceTypeRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let id = create_finance_type(&pool, &payload.name)
        .await
        .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn create_transaction_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<CreateTransactionRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let id = create_transaction(
        &pool,
        user_id,
        payload.account_id,
        payload.type_id,
        payload.amount,
        &payload.description,
    )
    .await
    .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn create_planned_expense_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<CreatePlannedExpenseRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let next_date = parse_date(&payload.next_date)?;
    let id = create_planned_expense(
        &pool,
        user_id,
        &payload.description,
        payload.amount,
        payload.account_id,
        payload.type_id,
        &payload.periodicite,
        payload.intervalle,
        next_date,
    )
    .await
    .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn create_habit_category_endpoint(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateHabitCategoryRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let id = create_habit_category(&pool, &payload.name)
        .await
        .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn create_habit_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<CreateHabitRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let id = create_habit(
        &pool,
        user_id,
        payload.category_id,
        &payload.title,
        &payload.description,
        &payload.habit_type,
    )
    .await
    .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn mark_habit_complete_endpoint(
    State(pool): State<DbPool>,
    Path((user_id, habit_id)): Path<(i32, i32)>,
    Json(payload): Json<CompleteHabitRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let date = parse_date(&payload.date)?;
    let id = mark_habit_complete(&pool, user_id, habit_id, date)
        .await
        .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn create_sobriety_period_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<CreateSobrietyPeriodRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let start_date = parse_date(&payload.start_date)?;
    let id = create_sobriety_period(&pool, user_id, start_date)
        .await
        .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn create_mood_type_endpoint(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateMoodTypeRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let id = create_mood_type(&pool, &payload.name)
        .await
        .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn log_mood_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<LogMoodRequest>,
) -> Result<StatusCode, (StatusCode, Json<ApiError>)> {
    let date = parse_date(&payload.date)?;
    log_mood(&pool, user_id, payload.type_id, date, payload.notes.as_deref())
        .await
        .map_err(today_dashboard_error)?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn log_hydration_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<LogHydrationRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let date = parse_date(&payload.date)?;
    let id = log_hydration(
        &pool,
        user_id,
        date,
        payload.quantity,
        &payload.hydration_type,
        payload.objective,
    )
    .await
    .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn log_sleep_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<LogSleepRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let date = parse_date(&payload.date)?;
    let id = log_sleep(
        &pool,
        user_id,
        date,
        &payload.time,
        payload.duration,
        payload.quality,
        payload.is_restful,
    )
    .await
    .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn log_meal_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<LogMealRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let date = parse_date(&payload.date)?;
    let id = log_meal(
        &pool,
        user_id,
        date,
        &payload.time,
        &payload.name,
        payload.calories,
        payload.proteins,
        payload.carbs,
        payload.fats,
    )
    .await
    .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn log_body_measurement_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<LogBodyMeasurementRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let date = parse_date(&payload.date)?;
    let id = log_body_measurement(
        &pool,
        user_id,
        date,
        payload.weight,
        payload.height,
        payload.chest,
        payload.waist,
        payload.hips,
    )
    .await
    .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn create_sport_type_endpoint(
    State(pool): State<DbPool>,
    Json(payload): Json<CreateSportTypeRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let id = create_sport_type(&pool, &payload.name)
        .await
        .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn log_sport_session_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<LogSportSessionRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let date = parse_date(&payload.date)?;
    let id = log_sport_session(
        &pool,
        user_id,
        payload.type_id,
        date,
        &payload.time,
        payload.duration,
        payload.calories,
        &payload.intensity,
    )
    .await
    .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn log_breathing_session_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<LogBreathingSessionRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let date = parse_date(&payload.date)?;
    let id = log_breathing_session(&pool, user_id, date, &payload.time, payload.duration, &payload.frequency)
        .await
        .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn log_alcohol_consumption_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<LogAlcoholConsumptionRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let date = parse_date(&payload.date)?;
    let id = log_alcohol_consumption(
        &pool,
        user_id,
        date,
        &payload.time,
        &payload.alcohol_type,
        payload.quantity,
        payload.percentage,
    )
    .await
    .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

pub async fn create_todo_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let _due_date = match payload.due_date.as_deref() {
        Some(value) => Some(parse_date(value)?),
        None => None,
    };
    let id = create_todo(&pool, user_id, &payload.title, payload.description.as_deref(), None)
        .await
        .map_err(today_dashboard_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

// ── Zero-Knowledge endpoints ─────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct EncryptedDateQuery {
    pub date: Option<String>,
}

/// POST /users/:user_id/encrypted
/// Reçoit { date, iv, ciphertext, version } — le serveur stocke iv + ciphertext
/// opaques sans jamais déchiffrer.
pub async fn save_encrypted_entry_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Json(payload): Json<EncryptedPayloadRequest>,
) -> Result<(StatusCode, Json<CreatedResponse>), (StatusCode, Json<ApiError>)> {
    let id = save_encrypted_entry(
        &pool,
        user_id,
        payload.date,
        &payload.iv,
        &payload.ciphertext,
        payload.version,
    )
    .await
    .map_err(encrypted_error)?;
    Ok((StatusCode::CREATED, Json(CreatedResponse { id })))
}

/// GET /users/:user_id/encrypted?date=YYYY-MM-DD
/// Retourne les entrées chiffrées pour une date précise.
pub async fn get_encrypted_entries_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
    Query(params): Query<EncryptedDateQuery>,
) -> Result<Json<Vec<EncryptedEntry>>, (StatusCode, Json<ApiError>)> {
    let date_str = params.date.ok_or_else(|| bad_request("Missing query param 'date'"))?;
    let date = parse_date(&date_str)?;
    get_encrypted_entries(&pool, user_id, date)
        .await
        .map(Json)
        .map_err(encrypted_error)
}

/// GET /users/:user_id/encrypted/all
/// Retourne toutes les entrées chiffrées de l'utilisateur.
pub async fn get_all_encrypted_entries_endpoint(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<EncryptedEntry>>, (StatusCode, Json<ApiError>)> {
    get_all_encrypted_entries(&pool, user_id)
        .await
        .map(Json)
        .map_err(encrypted_error)
}

/// GET /users/me
/// Récupère le profil de l'utilisateur connecté (protégé par JWT)
pub async fn get_current_user(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, (StatusCode, Json<ApiError>)> {
    get_user_by_id(&pool, user_id)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    message: "Database error".to_string(),
                }),
            )
        })?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError {
                    message: "User not found".to_string(),
                }),
            )
        })
        .map(Json)
}

pub async fn api_info() -> Json<ApiInfo> {
    Json(ApiInfo {
        name: "LifeTrack API".to_string(),
        description: "Comprehensive life tracking and personal wellness management API".to_string(),
        version: "1.0.0".to_string(),
        endpoints: vec![
            EndpointInfo {
                method: "GET".to_string(),
                path: "/health".to_string(),
                description: "Health check endpoint".to_string(),
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/api/info".to_string(),
                description: "API information and endpoints listing".to_string(),
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/users/:user_id/profile".to_string(),
                description: "Get user profile information".to_string(),
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/users/:user_id/dashboard/today".to_string(),
                description: "Get today's dashboard with daily metrics".to_string(),
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/users/:user_id/latest-module-values".to_string(),
                description: "Get latest values from all modules".to_string(),
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/users/:user_id/alerts-reminders".to_string(),
                description: "Get active alerts and reminders for user".to_string(),
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/users".to_string(),
                description: "Create a user".to_string(),
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/users/:user_id/accounts".to_string(),
                description: "Create an account for user".to_string(),
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/finance/types".to_string(),
                description: "Create a finance type".to_string(),
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/users/:user_id/transactions".to_string(),
                description: "Create a transaction for user".to_string(),
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/users/:user_id/planned-expenses".to_string(),
                description: "Create a planned expense for user".to_string(),
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/habits/categories".to_string(),
                description: "Create a habit category".to_string(),
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/users/:user_id/habits".to_string(),
                description: "Create a habit for user".to_string(),
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/users/:user_id/todos".to_string(),
                description: "Create a todo for user".to_string(),
            },
            EndpointInfo {
                method: "POST".to_string(),
                path: "/users/:user_id/encrypted".to_string(),
                description: "Store an AES-GCM encrypted payload (ZK — server never decrypts)".to_string(),
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/users/:user_id/encrypted?date=YYYY-MM-DD".to_string(),
                description: "Get encrypted entries for a specific date".to_string(),
            },
            EndpointInfo {
                method: "GET".to_string(),
                path: "/users/:user_id/encrypted/all".to_string(),
                description: "Get all encrypted entries for a user".to_string(),
            },
        ],
    })
}
