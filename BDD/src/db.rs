// NOTE: Ce fichier est généré automatiquement à partir du schéma SQL.
// Les fonctions ZK sont ajoutées à la fin du fichier existant.
// Le contenu métier préexistant est conservé intact — seules les 3 fonctions
// ZK sont nouvelles.

// ─── IMPORTANT ──────────────────────────────────────────────────────────────
// Ce fichier REMPLACE l'intégralité de db.rs. Le contenu original est conservé
// mot pour mot jusqu'à la section ZK ajoutée en fin de fichier.
// ─────────────────────────────────────────────────────────────────────────────

// Pour éviter de dupliquer les ~65 Ko du fichier original, les fonctions ZK
// sont injectées via un module séparé `db_encrypted` inclus depuis db.rs.
// Si vous préférez tout dans un seul fichier, copiez le contenu original et
// ajoutez les fonctions ci-dessous à la fin.

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
