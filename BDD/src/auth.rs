use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use sqlx::MySqlPool;

use crate::models::ApiError;

/// Middleware Axum : vérifie que le `UsrpublicId` transmis dans le header
/// `X-User-Public-Id` correspond à un utilisateur existant dans UTILISATEUR.
///
/// Usage dans routes.rs :
/// ```rust
/// use axum::middleware;
/// Router::new()
///     .route(...)
///     .layer(middleware::from_fn_with_state(pool.clone(), validate_user_by_public_id))
/// ```
pub async fn validate_user_by_public_id(
    State(pool): State<MySqlPool>,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ApiError>)> {
    let public_id = req
        .headers()
        .get("X-User-Public-Id")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    let public_id = match public_id {
        Some(id) if !id.is_empty() => id,
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    message: "Missing or empty X-User-Public-Id header".to_string(),
                }),
            ));
        }
    };

    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM UTILISATEUR WHERE UsrpublicId = ?)",
    )
    .bind(&public_id)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                message: format!("Auth DB error: {e}"),
            }),
        )
    })?;

    if exists {
        Ok(next.run(req).await)
    } else {
        Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                message: format!("Unknown public user id: {public_id}"),
            }),
        ))
    }
}

/// Stub legacy — conservé pour compatibilité ascendante.
#[allow(dead_code)]
pub fn check_auth() {
    println!("Checking auth");
}
