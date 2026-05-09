use sqlx::{Pool, MySql};
use sqlx::mysql::MySqlPoolOptions;

use crate::config::DbConfig;
use crate::models::{TodayDashboard, UserProfile};

pub type DbPool = Pool<MySql>;

pub async fn connect_db(cfg: &DbConfig) -> DbPool {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&cfg.url)
        .await
        .expect("Impossible de se connecter à la base MariaDB/MySQL")
}

pub async fn get_user_profile(pool: &DbPool, user_id: i32) -> Result<UserProfile, sqlx::Error> {
    sqlx::query_as::<_, UserProfile>(
        r#"
        SELECT
            u.Usrid AS usrid,
            u.UsrpublicId AS usrpublic_id,
            u.UsrcreatedAt AS usrcreated_at
        FROM UTILISATEUR u
        WHERE u.Usrid = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_today_dashboard(
    pool: &DbPool,
    user_id: i32,
) -> Result<TodayDashboard, sqlx::Error> {
    sqlx::query_as::<_, TodayDashboard>(
        r#"
        SELECT
            (SELECT COALESCE(SUM(h.Hydquantite), 0)
             FROM HYDRATATION h
             WHERE h.Usrid = ? AND h.Hyddate = CURDATE()) AS today_water_total,
            (SELECT COALESCE(SUM(r.Repcalories), 0)
             FROM REPAS r
             WHERE r.Usrid = ? AND r.Repdate = CURDATE()) AS today_calories_total,
            (SELECT COALESCE(SUM(r.Repproteines), 0)
             FROM REPAS r
             WHERE r.Usrid = ? AND r.Repdate = CURDATE()) AS today_protein_total,
            (SELECT COALESCE(SUM(r.Repglucides), 0)
             FROM REPAS r
             WHERE r.Usrid = ? AND r.Repdate = CURDATE()) AS today_carb_total,
            (SELECT COALESCE(SUM(r.Replipides), 0)
             FROM REPAS r
             WHERE r.Usrid = ? AND r.Repdate = CURDATE()) AS today_fat_total,
            (SELECT COALESCE(SUM(s.Seaduree), 0)
             FROM SEANCE_SPORT s
             WHERE s.Usrid = ? AND s.Seadate = CURDATE()) AS today_sport_duration,
            (SELECT COALESCE(COUNT(*), 0)
             FROM SEANCE_SPORT s
             WHERE s.Usrid = ? AND s.Seadate = CURDATE()) AS today_sport_count,
            (SELECT COALESCE(COUNT(*), 0)
             FROM TODO t
             WHERE t.Usrid = ? AND t.Toddone = 0) AS open_todos,
            (SELECT COALESCE(COUNT(*), 0)
             FROM BILAN b
             WHERE b.Usrid = ? AND b.Bildate = CURDATE()) AS today_bilan_count,
            (SELECT COALESCE(COUNT(*), 0)
             FROM DATE_HUMEUR dh
             WHERE dh.Usrid = ? AND dh.DHdate = CURDATE()) AS today_mood_count,
            (SELECT COALESCE(COUNT(*), 0)
             FROM SOMMEIL s
             WHERE s.Usrid = ? AND s.Somdate = CURDATE()) AS today_sleep_count
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}