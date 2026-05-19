// Authentification et middlewares JWT
//
// Description (FR):
// Ce module gère l'authentification: génération/validation de JWT,
// dérivation de clés, et le middleware `validate_jwt` utilisé pour protéger
// les routes nécessitant un utilisateur authentifié.
// Utilise le type d'erreur centralisé `AppError` pour un traitement cohérent.

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    Json,
    http::StatusCode,
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use chrono::Utc;
use hex;

use crate::errors::{AppError, Result};
use crate::models::{JwtClaims, AuthResponse, RegisterRequest, LoginRequest, DerivedKeysResponse};
use crate::password::{hash_passphrase, verify_passphrase, generate_salt, derive_encryption_key};
use crate::db::{self, DbPool};

/// Middleware Axum pour valider les JWT tokens
/// Retourne un `AppError::Unauthorized` si le token est invalide
pub async fn validate_jwt(
    State((pool, jwt_secret)): State<(DbPool, String)>,
    req: Request,
    next: Next,
) -> Result<Response> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_owned());

    let token = match auth_header {
        Some(ref header) if header.starts_with("Bearer ") => header[7..].to_owned(),
        _ => {
            return Err(AppError::Unauthorized(
                "Missing or invalid Authorization header".into(),
            ));
        }
    };

    let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let claims = decode::<JwtClaims>(&token, &decoding_key, &Validation::default())
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?
        .claims;

    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| AppError::Unauthorized("Invalid user ID in token".into()))?;

    match db::get_user_by_id(&pool, user_id).await? {
        Some(_) => Ok(next.run(req).await),
        None => Err(AppError::NotFound),
    }
}

/// Endpoint POST /auth/register
/// Enregistre un nouvel utilisateur, génère le salt AES une seule fois
/// et retourne la clé de chiffrement + salt dans la réponse
pub async fn register_endpoint(
    State((pool, jwt_secret)): State<(DbPool, String)>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<AuthResponse>)> {
    // Vérifier que l'email n'existe pas déjà
    if let Some(_) = db::get_user_by_email(&pool, &payload.email).await? {
        return Err(AppError::Conflict("Email déjà utilisé".into()));
    }

    // Hash de la passphrase
    let hash = hash_passphrase(&payload.passphrase)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Hashing error: {}", e)))?;

    // Générer le salt UNE SEULE FOIS à la création du compte
    let salt_bytes = generate_salt();
    let salt_hex = hex::encode(&salt_bytes);

    // Dériver la clé AES-256 via PBKDF2-HMAC-SHA256 (100 000 iterations)
    let encryption_key = derive_encryption_key(&payload.passphrase, &salt_bytes);

    // Persister email + hash Argon2 + salt dans la DB
    let user_id = db::create_user(&pool, &payload.email, &hash, &salt_hex).await?;

    let token = generate_jwt(user_id, &payload.email, &jwt_secret)?;

    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user_id,
            email: payload.email,
            encryption_key: Some(encryption_key), // retournée au client à la création
            encryption_salt: Some(salt_hex),       // client doit stocker localement
        }),
    ))
}

/// Endpoint POST /auth/login
/// Reconstruit la clé AES à partir du salt persisté en base + passphrase fournie
pub async fn login_endpoint(
    State((pool, jwt_secret)): State<(DbPool, String)>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>> {
    let user = db::get_user_by_email(&pool, &payload.email)
        .await?
        .ok_or(AppError::Unauthorized("Email ou mot de passe invalide".into()))?;

    let is_valid = verify_passphrase(&payload.passphrase, &user.passphrase_hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Verification error: {}", e)))?;

    if !is_valid {
        return Err(AppError::Unauthorized("Email ou mot de passe invalide".into()));
    }

    // Reconstruire la clé AES depuis le salt persisté — jamais stockée côté serveur
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
/// Déterministe: lit le salt depuis la DB au lieu d'en générer un nouveau
/// Permet à un client de retrouver sa clé sans passer par le login
pub async fn derive_keys_endpoint(
    State((pool, _jwt_secret)): State<(DbPool, String)>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<DerivedKeysResponse>> {
    let user = db::get_user_by_email(&pool, &payload.email)
        .await?
        .ok_or(AppError::NotFound)?;

    let salt_hex = user.encryption_salt.unwrap_or_default();
    let salt_bytes = hex::decode(&salt_hex).unwrap_or_default();
    let encryption_key = derive_encryption_key(&payload.passphrase, &salt_bytes);

    Ok(Json(DerivedKeysResponse {
        encryption_key,
        salt: salt_hex,
    }))
}

// ── Helpers ──────────────────────────────────────────────────────────────

/// Génère un JWT token pour l'utilisateur
fn generate_jwt(user_id: i32, email: &str, jwt_secret: &str) -> Result<String> {
    let now = Utc::now().timestamp();
    let claims = JwtClaims {
        sub: user_id.to_string(),
        email: email.to_string(),
        iat: now,
        exp: now + 86400 * 7, // 7 jours
    };
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());
    encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("Token generation failed: {}", e)))
}

#[allow(dead_code)]
pub fn check_auth() {
    println!("Checking auth");
}
