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
/// Vérifie que:
/// - Le header Authorization contient un token Bearer valide
/// - Le token JWT est signé correctement et non expiré
/// - L'utilisateur existe toujours en base de données
pub async fn validate_jwt(
    State((pool, jwt_secret)): State<(MySqlPool, String)>,
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ApiError>)> {
    // Extraire le header Authorization (format: "Bearer <token>")\n    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    // Vérifier que le header est présent et commence par "Bearer "\n    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],  // Skip "Bearer "
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    message: "Missing or invalid Authorization header".to_string(),
                }),
            ));
        }
    };

    // Décoder et valider le JWT avec la clé secrète\n    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let claims = match decode::<JwtClaims>(token, &decoding_key, &Validation::default()) {
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
/// Enregistre un nouvel utilisateur avec email et passphrase
pub async fn register_endpoint(
    State((pool, jwt_secret)): State<(MySqlPool, String)>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, Json<ApiError>)> {
    // Vérifier que l'utilisateur n'existe pas déjà
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

    // Hash la passphrase
    let hash = hash_passphrase(&payload.passphrase)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    message: format!("Hashing error: {}", e),
                }),
            )
        })?;

    // Créer l'utilisateur
    let user_id = db::create_user(&pool, &payload.email, &hash)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError {
                    message: "Failed to create user".to_string(),
                }),
            )
        })?;

    // Générer JWT
    let token = generate_jwt(user_id, &payload.email, &jwt_secret)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user_id,
            email: payload.email,
        }),
    ))
}

    /// Endpoint POST /auth/login
    /// Authentifie l'utilisateur et retourne un JWT token
    pub async fn login_endpoint(
        State((pool, jwt_secret)): State<(MySqlPool, String)>,
        Json(payload): Json<LoginRequest>,
    ) -> Result<Json<AuthResponse>, (StatusCode, Json<ApiError>)> {
        // Récupérer l'utilisateur depuis la base de données par email
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
    
        // Vérifier que la passphrase correspond au hash stocké
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
    
        // Générer un nouveau JWT token
        let token = generate_jwt(user.usrid, &user.email, &jwt_secret)?;
    
        Ok(Json(AuthResponse {
            token,
            user_id: user.usrid,
            email: user.email,
        }))
    }
    
    /// Endpoint POST /auth/derive-keys
    /// Dérive une clé de chiffrement AES-256 à partir de la passphrase
    /// Retourne la clé + un salt aléatoire pour la reconstruction côté client
    pub async fn derive_keys_endpoint(
        Json(payload): Json<crate::models::PassphraseSetupRequest>,
    ) -> Json<DerivedKeysResponse> {
        // Générer un salt aléatoire sécurisé (16 bytes)\n        let salt = generate_salt();
        // Dériver la clé avec PBKDF2-SHA256\n        let encryption_key = derive_encryption_key(&payload.passphrase, &salt);
    
        Json(DerivedKeysResponse {
            encryption_key,
            salt: hex::encode(&salt),
        })
    }

// ── Helpers ─────────────────────────────────────────────────────────────────

fn generate_jwt(user_id: i32, email: &str, jwt_secret: &str) -> Result<String, (StatusCode, Json<ApiError>)> {
    let now = Utc::now().timestamp();
    let claims = JwtClaims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iat: now,
        exp: now + 86400 * 7,  // 7 jours
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

/// Stub legacy — conservé pour compatibilité ascendante.
#[allow(dead_code)]
pub fn check_auth() {
    println!("Checking auth");
}
