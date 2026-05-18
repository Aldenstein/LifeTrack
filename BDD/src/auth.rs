use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::Utc;
use sqlx::MySqlPool;
use hex;

use crate::models::{ApiError, JwtClaims, AuthResponse, RegisterRequest, LoginRequest, DerivedKeysResponse};
use crate::password::{hash_passphrase, verify_passphrase, generate_salt, derive_encryption_key};
use crate::db;

/// Middleware Axum pour valider les JWT tokens
pub async fn validate_jwt(
    State((pool, jwt_secret)): State<(MySqlPool, String)>,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ApiError>)> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    let token = match auth_header {
        Some(ref header) if header.starts_with("Bearer ") => header[7..].to_owned(),
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    message: "Missing or invalid Authorization header".to_string(),
                }),
            ));
        }
    };

    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let claims = match decode::<JwtClaims>(&token, &decoding_key, &Validation::default()) {
        Ok(data) => data.claims,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    message: "Invalid or expired token".to_string(),
                }),
            ));
        }
    };

    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    message: "Invalid user ID in token".to_string(),
                }),
            )
        })?;

    match db::get_user_by_id(&pool, user_id).await {
        Ok(Some(_)) => Ok(next.run(req).await),
        Ok(None) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                message: "User not found".to_string(),
            }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                message: "Database error during auth".to_string(),
            }),
        )),
    }
}

/// Endpoint POST /auth/register
/// Enregistre un nouvel utilisateur, genere le salt AES une seule fois
/// et retourne la cle de chiffrement + salt dans la reponse
pub async fn register_endpoint(
    State((pool, jwt_secret)): State<(MySqlPool, String)>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<ApiError>)> {
    match db::get_user_by_email(&pool, &payload.email).await {
        Ok(Some(_)) => {
            return Err((
                StatusCode::CONFLICT,
                Json(ApiError {
                    message: "User already exists".to_string(),
                }),
            ))
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    message: "Database error".to_string(),
                }),
            ))
        }
        Ok(None) => {}
    }

    let hash = hash_passphrase(&payload.passphrase)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    message: format!("Hashing error: {}", e),
                }),
            )
        })?;

    // Generer le salt UNE SEULE FOIS a la creation du compte
    let salt_bytes = generate_salt();
    let salt_hex = hex::encode(&salt_bytes);

    // Deriver la cle AES-256 via PBKDF2-HMAC-SHA256 (100 000 iterations)
    let encryption_key = derive_encryption_key(&payload.passphrase, &salt_bytes);

    // Persister email + hash Argon2 + salt dans la DB
    let user_id = db::create_user(&pool, &payload.email, &hash, &salt_hex)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    message: "Failed to create user".to_string(),
                }),
            )
        })?;

    let token = generate_jwt(user_id, &payload.email, &jwt_secret)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user_id,
            email: payload.email,
            encryption_key: Some(encryption_key), // retournee au client a la creation
            encryption_salt: Some(salt_hex),       // client doit stocker localement
        }),
    ))
}

/// Endpoint POST /auth/login
/// Reconstruit la cle AES a partir du salt persiste en base + passphrase fournie
pub async fn login_endpoint(
    State((pool, jwt_secret)): State<(MySqlPool, String)>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
    let user = db::get_user_by_email(&pool, &payload.email)
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
                StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    message: "Invalid email or password".to_string(),
                }),
            )
        })?;

    let is_valid = verify_passphrase(&payload.passphrase, &user.passphrase_hash)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    message: "Verification error".to_string(),
                }),
            )
        })?;

    if !is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                message: "Invalid email or password".to_string(),
            }),
        ));
    }

    // Reconstruire la cle AES depuis le salt persiste — jamais stockee cote serveur
    let encryption_key = user.encryption_salt.as_ref().map(|salt_hex| {
        let salt_bytes = hex::decode(salt_hex).unwrap_or_default();
        derive_encryption_key(&payload.passphrase, &salt_bytes)
    });

    let token = generate_jwt(user.usrid, &user.email, &jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user_id: user.usrid,
        email: user.email,
        encryption_key,
        encryption_salt: user.encryption_salt,
    }))
}

/// Endpoint POST /auth/derive-keys
/// Desormais deterministe : lit le salt depuis la DB au lieu d'en generer un nouveau
/// Permet a un client de retrouver sa cle sans passer par le login
pub async fn derive_keys_endpoint(
    State((pool, _jwt_secret)): State<(MySqlPool, String)>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<DerivedKeysResponse>, (StatusCode, Json<ApiError>)> {
    let user = db::get_user_by_email(&pool, &payload.email)
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
        })?;

    let salt_hex = user.encryption_salt.unwrap_or_default();
    let salt_bytes = hex::decode(&salt_hex).unwrap_or_default();
    let encryption_key = derive_encryption_key(&payload.passphrase, &salt_bytes);

    Ok(Json(DerivedKeysResponse {
        encryption_key,
        salt: salt_hex,
    }))
}

// -- Helpers --
fn generate_jwt(
    user_id: i32,
    email: &str,
    jwt_secret: &str,
) -> Result<String, (StatusCode, Json<ApiError>)> {
    let now = Utc::now().timestamp();
    let claims = JwtClaims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iat: now,
        exp: now + 86400 * 7, // 7 jours
    };
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    message: "Token generation failed".to_string(),
                }),
            )
        })
}

#[allow(dead_code)]
pub fn check_auth() {
    println!("Checking auth");
}
