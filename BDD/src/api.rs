use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::fmt::Display;
use serde::{Serialize, Deserialize};

use crate::db::{DbPool, get_today_dashboard, get_user_profile, get_latest_module_values, get_active_alerts_and_reminders};
use crate::models::{TodayDashboard, UserProfile, LatestModuleValues, ActiveAlert};

#[derive(Debug, Serialize)]
pub struct ApiError {
    pub message: String,
}

fn user_profile_error<E: Display>(error: E) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::NOT_FOUND,
        Json(ApiError {
            message: format!("Unable to load user profile: {error}"),
        }),
    )
}

fn today_dashboard_error<E: Display>(error: E) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            message: format!("Unable to load today dashboard: {error}"),
        }),
    )
}

fn latest_module_values_error<E: Display>(error: E) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            message: format!("Unable to load latest module values: {error}"),
        }),
    )
}

fn alerts_reminders_error<E: Display>(error: E) -> (StatusCode, Json<ApiError>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ApiError {
            message: format!("Unable to load alerts and reminders: {error}"),
        }),
    )
}

async fn map_user_profile_result(
    result: Result<UserProfile, sqlx::Error>,
) -> Result<Json<UserProfile>, (StatusCode, Json<ApiError>)> {
    result.map(Json).map_err(user_profile_error)
}

async fn map_today_dashboard_result(
    result: Result<TodayDashboard, sqlx::Error>,
) -> Result<Json<TodayDashboard>, (StatusCode, Json<ApiError>)> {
    result.map(Json).map_err(today_dashboard_error)
}

async fn map_latest_module_values_result(
    result: Result<Vec<LatestModuleValues>, sqlx::Error>,
) -> Result<Json<Vec<LatestModuleValues>>, (StatusCode, Json<ApiError>)> {
    result.map(Json).map_err(latest_module_values_error)
}

async fn map_alerts_reminders_result(
    result: Result<Vec<ActiveAlert>, sqlx::Error>,
) -> Result<Json<Vec<ActiveAlert>>, (StatusCode, Json<ApiError>)> {
    result.map(Json).map_err(alerts_reminders_error)
}

pub async fn user_profile(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<UserProfile>, (StatusCode, Json<ApiError>)> {
    map_user_profile_result(get_user_profile(&pool, user_id).await).await
}

pub async fn today_dashboard(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<TodayDashboard>, (StatusCode, Json<ApiError>)> {
    map_today_dashboard_result(get_today_dashboard(&pool, user_id).await).await
}

pub async fn latest_module_values(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<LatestModuleValues>>, (StatusCode, Json<ApiError>)> {
    map_latest_module_values_result(get_latest_module_values(&pool, user_id).await).await
}

pub async fn active_alerts_and_reminders(
    State(pool): State<DbPool>,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<ActiveAlert>>, (StatusCode, Json<ApiError>)> {
    map_alerts_reminders_result(get_active_alerts_and_reminders(&pool, user_id).await).await
}

pub fn start_api() {
    println!("API started");
}

// ===== HEALTH & INFO ENDPOINTS =====

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub version: String,
    pub timestamp: String,
}

pub async fn health_check() -> Json<HealthStatus> {
    Json(HealthStatus {
        status: "healthy".to_string(),
        version: "1.0.0".to_string(),
        timestamp: chrono::Local::now().to_rfc3339(),
    })
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
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use serde_json::json;

    fn sample_profile() -> UserProfile {
        UserProfile {
            usrid: 7,
            usrpublic_id: "public-7".to_string(),
            usrcreated_at: NaiveDate::from_ymd_opt(2026, 5, 10)
                .expect("valid date")
                .and_hms_opt(12, 30, 0)
                .expect("valid time"),
        }
    }

    fn sample_dashboard() -> TodayDashboard {
        TodayDashboard {
            today_water_total: 1.5,
            today_calories_total: 2200.0,
            today_protein_total: 120.0,
            today_carb_total: 260.0,
            today_fat_total: 70.0,
            today_sport_duration: 45,
            today_sport_count: 2,
            open_todos: 4,
            today_bilan_count: 1,
            today_mood_count: 1,
            today_sleep_count: 1,
        }
    }

    #[tokio::test]
    async fn user_profile_success_is_returned_as_json() {
        let result = map_user_profile_result(Ok(sample_profile())).await;

        let response = result.expect("expected success response");
        let body = serde_json::to_value(response.0).expect("serializable profile");

        assert_eq!(body, json!({
            "usrid": 7,
            "usrpublic_id": "public-7",
            "usrcreated_at": "2026-05-10T12:30:00"
        }));
    }

    #[tokio::test]
    async fn user_profile_error_is_mapped_to_not_found() {
        let result = map_user_profile_result(Err(sqlx::Error::RowNotFound)).await;

        let (status, Json(error)) = result.expect_err("expected an error response");

        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(error.message.contains("Unable to load user profile"));
    }

    #[tokio::test]
    async fn dashboard_success_is_returned_as_json() {
        let result = map_today_dashboard_result(Ok(sample_dashboard())).await;

        let response = result.expect("expected success response");
        let body = serde_json::to_value(response.0).expect("serializable dashboard");

        assert_eq!(body, json!({
            "today_water_total": 1.5,
            "today_calories_total": 2200.0,
            "today_protein_total": 120.0,
            "today_carb_total": 260.0,
            "today_fat_total": 70.0,
            "today_sport_duration": 45,
            "today_sport_count": 2,
            "open_todos": 4,
            "today_bilan_count": 1,
            "today_mood_count": 1,
            "today_sleep_count": 1
        }));
    }

    #[tokio::test]
    async fn dashboard_error_is_mapped_to_server_error() {
        let result = map_today_dashboard_result(Err(sqlx::Error::RowNotFound)).await;

        let (status, Json(error)) = result.expect_err("expected an error response");

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(error.message.contains("Unable to load today dashboard"));
    }

    #[test]
    fn api_error_serializes_to_json() {
        let body = serde_json::to_value(ApiError {
            message: "sample failure".to_string(),
        })
        .expect("serializable error");

        assert_eq!(body, json!({ "message": "sample failure" }));
    }

    #[tokio::test]
    async fn health_check_returns_healthy_status() {
        let response = health_check().await;
        let body = serde_json::to_value(response.0).expect("serializable health check");

        assert_eq!(body["status"], "healthy");
        assert_eq!(body["version"], "1.0.0");
        assert!(body["timestamp"].is_string());
    }

    #[tokio::test]
    async fn api_info_contains_all_endpoints() {
        let response = api_info().await;
        let body = response.0;

        assert_eq!(body.name, "LifeTrack API");
        assert_eq!(body.version, "1.0.0");
        assert!(body.endpoints.len() >= 6);

        let paths: Vec<String> = body.endpoints.iter().map(|e| e.path.clone()).collect();
        assert!(paths.contains(&"/health".to_string()));
        assert!(paths.contains(&"/api/info".to_string()));
        assert!(paths.contains(&"/users/:user_id/profile".to_string()));
        assert!(paths.contains(&"/users/:user_id/dashboard/today".to_string()));
        assert!(paths.contains(&"/users/:user_id/latest-module-values".to_string()));
        assert!(paths.contains(&"/users/:user_id/alerts-reminders".to_string()));
    }

    #[tokio::test]
    async fn latest_module_values_error_is_mapped_to_server_error() {
        let result = map_latest_module_values_result(Err(sqlx::Error::RowNotFound)).await;

        let (status, Json(error)) = result.expect_err("expected an error response");

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(error.message.contains("Unable to load latest module values"));
    }

    #[tokio::test]
    async fn alerts_reminders_error_is_mapped_to_server_error() {
        let result = map_alerts_reminders_result(Err(sqlx::Error::RowNotFound)).await;

        let (status, Json(error)) = result.expect_err("expected an error response");

        assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);
        assert!(error.message.contains("Unable to load alerts and reminders"));
    }
}