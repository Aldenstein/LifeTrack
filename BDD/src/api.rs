use axum::{extract::{Path, State}, http::StatusCode, Json};
use std::fmt::Display;
use serde::Serialize;

use crate::db::{DbPool, get_today_dashboard, get_user_profile};
use crate::models::{TodayDashboard, UserProfile};

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

pub fn start_api() {
    println!("API started");
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
}