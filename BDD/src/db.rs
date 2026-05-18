use sqlx::{MySqlPool, Row};
use chrono::NaiveDate;
use crate::models::EncryptedEntry;

pub type DbPool = MySqlPool;

// ── Zero-Knowledge : DONNEE_CHIFFREE ────────────────────────────────────────

/// Insère une entrée chiffrée pour un utilisateur.
/// Retourne le Dcid auto-incrémenté.
pub async fn save_encrypted_entry(
    pool: &MySqlPool,
    user_id: i32,
    date: NaiveDate,
    iv: &str,
    ciphertext: &str,
    version: i32,
) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO DONNEE_CHIFFREE (Usrid, Dcdate, Dciv, Dcciphertext, Dcversion) \
         VALUES (?, ?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(date)
    .bind(iv)
    .bind(ciphertext)
    .bind(version)
    .execute(pool)
    .await?;

    Ok(result.last_insert_id() as i32)
}

/// Récupère toutes les entrées chiffrées d'un utilisateur pour une date donnée.
pub async fn get_encrypted_entries(
    pool: &MySqlPool,
    user_id: i32,
    date: NaiveDate,
) -> Result<Vec<EncryptedEntry>, sqlx::Error> {
    sqlx::query_as::<_, EncryptedEntry>(
        "SELECT Dcid as dcid, Usrid as usrid, Dcdate as dcdate, \
                Dciv as dciv, Dcciphertext as dcciphertext, Dcversion as dcversion \
         FROM DONNEE_CHIFFREE \
         WHERE Usrid = ? AND Dcdate = ? \
         ORDER BY Dcid ASC",
    )
    .bind(user_id)
    .bind(date)
    .fetch_all(pool)
    .await
}

/// Récupère toutes les entrées chiffrées d'un utilisateur (toutes dates).
pub async fn get_all_encrypted_entries(
    pool: &MySqlPool,
    user_id: i32,
) -> Result<Vec<EncryptedEntry>, sqlx::Error> {
    sqlx::query_as::<_, EncryptedEntry>(
        "SELECT Dcid as dcid, Usrid as usrid, Dcdate as dcdate, \
                Dciv as dciv, Dcciphertext as dcciphertext, Dcversion as dcversion \
         FROM DONNEE_CHIFFREE \
         WHERE Usrid = ? \
         ORDER BY Dcdate DESC, Dcid ASC",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}
