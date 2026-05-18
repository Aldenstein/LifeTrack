use sqlx::{Pool, MySql};
use sqlx::mysql::MySqlPoolOptions;
use chrono::NaiveDate;

use crate::config::DbConfig;
use crate::models::*;

pub type DbPool = Pool<MySql>;

pub async fn connect_db(cfg: &DbConfig) -> DbPool {
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&cfg.url)
        .await
        .expect("Impossible de se connecter à la base MariaDB/MySQL")
}

// ── Zero-Knowledge : DONNEE_CHIFFREE ────────────────────────────────────────

/// Insère une entrée chiffrée pour un utilisateur.
/// Retourne le Dcid auto-incrémenté.
pub async fn save_encrypted_entry(
        pool: &DbPool,
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
        pool: &DbPool,
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
        pool: &DbPool,
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

// ===== GLOBAL FUNCTIONS =====

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

pub async fn get_latest_module_values(pool: &DbPool, user_id: i32) -> Result<Vec<LatestModuleValues>, sqlx::Error> {
    sqlx::query_as::<_, LatestModuleValues>(
        r#"
        SELECT 'HYDRATATION' AS module_name, 'water_total' AS metric_name,
               CAST(h.Hydquantite AS CHAR) AS metric_value, CAST(h.Hyddate AS CHAR) AS metric_date
        FROM HYDRATATION h
        WHERE h.Usrid = ?
        ORDER BY h.Hyddate DESC, h.Hydid DESC
        LIMIT 1
        UNION ALL
        SELECT 'SOMMEIL', 'sleep_duration', CAST(s.Somduree AS CHAR), CAST(s.Somdate AS CHAR)
        FROM SOMMEIL s
        WHERE s.Usrid = ?
        ORDER BY s.Somdate DESC, s.Somid DESC
        LIMIT 1
        UNION ALL
        SELECT 'REPAS', 'calories', CAST(r.Repcalories AS CHAR), CAST(r.Repdate AS CHAR)
        FROM REPAS r
        WHERE r.Usrid = ?
        ORDER BY r.Repdate DESC, r.Repid DESC
        LIMIT 1
        UNION ALL
        SELECT 'SEANCE_SPORT', 'calories', CAST(s.Seacalories AS CHAR), CAST(s.Seadate AS CHAR)
        FROM SEANCE_SPORT s
        WHERE s.Usrid = ?
        ORDER BY s.Seadate DESC, s.Seaid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_active_alerts_and_reminders(pool: &DbPool, user_id: i32) -> Result<Vec<ActiveAlert>, sqlx::Error> {
    sqlx::query_as::<_, ActiveAlert>(
        r#"
        SELECT 'FACTURE' AS alert_type, f.Facid AS item_id, f.FacdateProchain AS due_date
        FROM FACTURE f
        WHERE f.Usrid = ?
          AND f.Facdone = 0
          AND f.FacdateProchain IS NOT NULL
          AND f.FacdateProchain <= CURDATE()
        UNION ALL
        SELECT 'TODO', t.Todid, NULL
        FROM TODO t
        WHERE t.Usrid = ?
          AND t.Toddone = 0
        UNION ALL
        SELECT 'HYDRATATION', h.Hydid, h.Hyddate
        FROM HYDRATATION h
        WHERE h.Usrid = ?
          AND h.Hyddate = CURDATE()
          AND h.Hydquantite < h.Hydobjectif
        UNION ALL
        SELECT 'SOMMEIL', s.Somid, s.Somdate
        FROM SOMMEIL s
        WHERE s.Usrid = ?
          AND s.Somdate = CURDATE()
          AND s.Somduree IS NOT NULL
          AND s.Somduree < 420
        UNION ALL
        SELECT 'DATE_HUMEUR', dh.Usrid, dh.DHdate
        FROM DATE_HUMEUR dh
        WHERE dh.Usrid = ?
          AND dh.DHdate = CURDATE()
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .bind(user_id)
    .fetch_all(pool)
    .await
}

// ===== FINANCES FUNCTIONS =====

pub async fn get_user_accounts(pool: &DbPool, user_id: i32) -> Result<Vec<Account>, sqlx::Error> {
    sqlx::query_as::<_, Account>(
        r#"
        SELECT c.Comid AS comid, c.Usrid AS usrid, c.Comnom AS comnom, c.Comsolde AS comsolde
        FROM COMPTE c
        WHERE c.Usrid = ?
        ORDER BY c.Comnom ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_account_balances(pool: &DbPool, user_id: i32) -> Result<Vec<AccountBalance>, sqlx::Error> {
    sqlx::query_as::<_, AccountBalance>(
        r#"
        SELECT c.Comid AS comid, c.Comnom AS comnom, c.Comsolde AS comsolde
        FROM COMPTE c
        WHERE c.Usrid = ?
        ORDER BY c.Comnom ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_finance_types(pool: &DbPool) -> Result<Vec<FinanceType>, sqlx::Error> {
    sqlx::query_as::<_, FinanceType>(
        r#"
        SELECT t.Typid AS typid, t.Typtitre AS typtitre
        FROM TYPE t
        ORDER BY t.Typtitre ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_transactions_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<Transaction>, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(
        r#"
        SELECT m.Mouid AS mouid, m.Usrid AS usrid, m.Comid AS comid, m.Typid AS typid,
               m.Moudate AS moudate, m.Moumontant AS moumontant, m.Moudescription AS moudescription
        FROM MOUVEMENT m
        WHERE m.Usrid = ?
          AND m.Moudate BETWEEN ? AND ?
        ORDER BY m.Moudate DESC, m.Mouid DESC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

pub async fn get_transactions_by_account(
    pool: &DbPool,
    user_id: i32,
    account_id: i32,
) -> Result<Vec<Transaction>, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(
        r#"
        SELECT m.Mouid AS mouid, m.Usrid AS usrid, m.Comid AS comid, m.Typid AS typid,
               m.Moudate AS moudate, m.Moumontant AS moumontant, m.Moudescription AS moudescription
        FROM MOUVEMENT m
        WHERE m.Usrid = ?
          AND m.Comid = ?
        ORDER BY m.Moudate DESC, m.Mouid DESC
        "#,
    )
    .bind(user_id)
    .bind(account_id)
    .fetch_all(pool)
    .await
}

pub async fn get_transactions_by_type(
    pool: &DbPool,
    user_id: i32,
    type_id: i32,
) -> Result<Vec<Transaction>, sqlx::Error> {
    sqlx::query_as::<_, Transaction>(
        r#"
        SELECT m.Mouid AS mouid, m.Usrid AS usrid, m.Comid AS comid, m.Typid AS typid,
               m.Moudate AS moudate, m.Moumontant AS moumontant, m.Moudescription AS moudescription
        FROM MOUVEMENT m
        WHERE m.Usrid = ?
          AND m.Typid = ?
        ORDER BY m.Moudate DESC, m.Mouid DESC
        "#,
    )
    .bind(user_id)
    .bind(type_id)
    .fetch_all(pool)
    .await
}

pub async fn get_income_total_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<IncomeTotal, sqlx::Error> {
    sqlx::query_as::<_, IncomeTotal>(
        r#"
        SELECT COALESCE(SUM(CASE WHEN m.Moumontant > 0 THEN m.Moumontant ELSE 0 END), 0) AS income_total
        FROM MOUVEMENT m
        WHERE m.Usrid = ?
          AND m.Moudate BETWEEN ? AND ?
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await
}

pub async fn get_expense_total_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<ExpenseTotal, sqlx::Error> {
    sqlx::query_as::<_, ExpenseTotal>(
        r#"
        SELECT COALESCE(SUM(CASE WHEN m.Moumontant < 0 THEN ABS(m.Moumontant) ELSE 0 END), 0) AS expense_total
        FROM MOUVEMENT m
        WHERE m.Usrid = ?
          AND m.Moudate BETWEEN ? AND ?
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await
}

pub async fn get_net_balance_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<NetBalance, sqlx::Error> {
    sqlx::query_as::<_, NetBalance>(
        r#"
        SELECT COALESCE(SUM(m.Moumontant), 0) AS net_balance
        FROM MOUVEMENT m
        WHERE m.Usrid = ?
          AND m.Moudate BETWEEN ? AND ?
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await
}

pub async fn get_planned_expenses(pool: &DbPool, user_id: i32) -> Result<Vec<PlannedExpense>, sqlx::Error> {
    sqlx::query_as::<_, PlannedExpense>(
        r#"
        SELECT f.Facid AS facid, f.Usrid AS usrid, f.Comid AS comid, f.Mouid AS mouid,
               f.Facdate AS facdate, f.Facperiodicite AS facperiodicite,
               f.Facintervalle AS facintervalle, f.FacdateProchain AS facdateprochain,
               f.Facdone AS facdone,
               m.Moumontant AS moumontant, m.Moudescription AS moudescription
        FROM FACTURE f
        JOIN MOUVEMENT m ON m.Mouid = f.Mouid
        WHERE f.Usrid = ?
        ORDER BY f.FacdateProchain ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_upcoming_planned_expenses(pool: &DbPool, user_id: i32) -> Result<Vec<PlannedExpense>, sqlx::Error> {
    sqlx::query_as::<_, PlannedExpense>(
        r#"
        SELECT f.Facid AS facid, f.Usrid AS usrid, f.Comid AS comid, f.Mouid AS mouid,
               f.Facdate AS facdate, f.Facperiodicite AS facperiodicite,
               f.Facintervalle AS facintervalle, f.FacdateProchain AS facdateprochain,
               f.Facdone AS facdone,
               m.Moumontant AS moumontant, m.Moudescription AS moudescription
        FROM FACTURE f
        JOIN MOUVEMENT m ON m.Mouid = f.Mouid
        WHERE f.Usrid = ?
          AND f.FacdateProchain IS NOT NULL
          AND f.FacdateProchain >= CURDATE()
        ORDER BY f.FacdateProchain ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_top_expense_types(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    limit: i32,
) -> Result<Vec<TopExpenseType>, sqlx::Error> {
    sqlx::query_as::<_, TopExpenseType>(
        r#"
        SELECT t.Typid AS typid, t.Typtitre AS typtitre,
               COALESCE(SUM(ABS(m.Moumontant)), 0) AS total_expense
        FROM MOUVEMENT m
        JOIN TYPE t ON t.Typid = m.Typid
        WHERE m.Usrid = ?
          AND m.Moudate BETWEEN ? AND ?
          AND m.Moumontant < 0
        GROUP BY t.Typid, t.Typtitre
        ORDER BY total_expense DESC
        LIMIT ?
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn get_balance_history(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<BalanceHistory>, sqlx::Error> {
    sqlx::query_as::<_, BalanceHistory>(
        r#"
        SELECT m.Moudate AS moudate, SUM(m.Moumontant) OVER (ORDER BY m.Moudate, m.Mouid) AS running_balance
        FROM MOUVEMENT m
        WHERE m.Usrid = ?
          AND m.Moudate BETWEEN ? AND ?
        ORDER BY m.Moudate ASC, m.Mouid ASC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

// ===== HABITS FUNCTIONS =====

pub async fn get_habit_categories(pool: &DbPool) -> Result<Vec<HabitCategory>, sqlx::Error> {
    sqlx::query_as::<_, HabitCategory>(
        r#"
        SELECT c.Catid AS catid, c.Catnom AS catnom, c.Catplus AS catplus
        FROM CATEGORIE c
        ORDER BY c.Catnom ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_active_habits(pool: &DbPool, user_id: i32) -> Result<Vec<Habit>, sqlx::Error> {
    sqlx::query_as::<_, Habit>(
        r#"
        SELECT h.Habid AS habid, h.Usrid AS usrid, h.Catid AS catid, h.Habnom AS habnom
        FROM HABITUDE h
        WHERE h.Usrid = ?
        ORDER BY h.Habnom ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_positive_habits(pool: &DbPool, user_id: i32) -> Result<Vec<Habit>, sqlx::Error> {
    sqlx::query_as::<_, Habit>(
        r#"
        SELECT h.Habid AS habid, h.Usrid AS usrid, h.Catid AS catid, h.Habnom AS habnom
        FROM HABITUDE h
        JOIN CATEGORIE c ON c.Catid = h.Catid
        WHERE h.Usrid = ?
          AND c.Catplus = '1'
        ORDER BY h.Habnom ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_negative_habits(pool: &DbPool, user_id: i32) -> Result<Vec<Habit>, sqlx::Error> {
    sqlx::query_as::<_, Habit>(
        r#"
        SELECT h.Habid AS habid, h.Usrid AS usrid, h.Catid AS catid, h.Habnom AS habnom
        FROM HABITUDE h
        JOIN CATEGORIE c ON c.Catid = h.Catid
        WHERE h.Usrid = ?
          AND c.Catplus = '-1'
        ORDER BY h.Habnom ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_today_habits(pool: &DbPool, user_id: i32) -> Result<Vec<TodayHabit>, sqlx::Error> {
    sqlx::query_as::<_, TodayHabit>(
        r#"
        SELECT h.Habid AS habid, h.Habnom AS habnom, c.Catnom AS catnom, c.Catplus AS catplus,
               COALESCE(hb.HBdone, 0) AS done_today
        FROM HABITUDE h
        LEFT JOIN CATEGORIE c ON c.Catid = h.Catid
        LEFT JOIN BILAN b ON b.Usrid = h.Usrid AND b.Bildate = CURDATE()
        LEFT JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid AND hb.Habid = h.Habid
        WHERE h.Usrid = ?
        ORDER BY h.Habnom ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_completed_habits_today(pool: &DbPool, user_id: i32) -> Result<Vec<CompletedHabit>, sqlx::Error> {
    sqlx::query_as::<_, CompletedHabit>(
        r#"
        SELECT h.Habid AS habid, h.Habnom AS habnom, c.Catnom AS catnom, c.Catplus AS catplus
        FROM HABITUDE h
        JOIN CATEGORIE c ON c.Catid = h.Catid
        JOIN BILAN b ON b.Usrid = h.Usrid AND b.Bildate = CURDATE()
        JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid AND hb.Habid = h.Habid
        WHERE h.Usrid = ?
          AND hb.HBdone = 1
        ORDER BY h.Habnom ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_pending_habits_today(pool: &DbPool, user_id: i32) -> Result<Vec<CompletedHabit>, sqlx::Error> {
    sqlx::query_as::<_, CompletedHabit>(
        r#"
        SELECT h.Habid AS habid, h.Habnom AS habnom, c.Catnom AS catnom, c.Catplus AS catplus
        FROM HABITUDE h
        JOIN CATEGORIE c ON c.Catid = h.Catid
        LEFT JOIN BILAN b ON b.Usrid = h.Usrid AND b.Bildate = CURDATE()
        LEFT JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid AND hb.Habid = h.Habid
        WHERE h.Usrid = ?
          AND COALESCE(hb.HBdone, 0) = 0
        ORDER BY h.Habnom ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_today_habit_summary(pool: &DbPool, user_id: i32) -> Result<HabitSummary, sqlx::Error> {
    sqlx::query_as::<_, HabitSummary>(
        r#"
        SELECT
            COUNT(*) AS total_habits,
            COALESCE(SUM(CASE WHEN COALESCE(hb.HBdone, 0) = 1 THEN 1 ELSE 0 END), 0) AS completed_habits,
            COALESCE(SUM(CASE WHEN COALESCE(hb.HBdone, 0) = 0 THEN 1 ELSE 0 END), 0) AS pending_habits
        FROM HABITUDE h
        LEFT JOIN BILAN b ON b.Usrid = h.Usrid AND b.Bildate = CURDATE()
        LEFT JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid AND hb.Habid = h.Habid
        WHERE h.Usrid = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_today_habit_score(pool: &DbPool, user_id: i32) -> Result<HabitScore, sqlx::Error> {
    sqlx::query_as::<_, HabitScore>(
        r#"
        SELECT
            COALESCE(SUM(CASE
                WHEN c.Catplus = '1' AND COALESCE(hb.HBdone, 0) = 1 THEN 1
                WHEN c.Catplus = '-1' AND COALESCE(hb.HBdone, 0) = 0 THEN 1
                ELSE 0
            END), 0) AS habit_score
        FROM HABITUDE h
        LEFT JOIN CATEGORIE c ON c.Catid = h.Catid
        LEFT JOIN BILAN b ON b.Usrid = h.Usrid AND b.Bildate = CURDATE()
        LEFT JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid AND hb.Habid = h.Habid
        WHERE h.Usrid = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_weekly_habit_score(pool: &DbPool, user_id: i32) -> Result<Vec<WeeklyHabitScore>, sqlx::Error> {
    sqlx::query_as::<_, WeeklyHabitScore>(
        r#"
        SELECT
            b.Bildate AS bildate,
            COALESCE(SUM(CASE
                WHEN c.Catplus = '1' AND hb.HBdone = 1 THEN 1
                WHEN c.Catplus = '-1' AND hb.HBdone = 0 THEN 1
                ELSE 0
            END), 0) AS habit_score
        FROM BILAN b
        JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid
        JOIN HABITUDE h ON h.Habid = hb.Habid
        LEFT JOIN CATEGORIE c ON c.Catid = h.Catid
        WHERE b.Usrid = ?
          AND b.Bildate BETWEEN DATE_SUB(CURDATE(), INTERVAL 6 DAY) AND CURDATE()
        GROUP BY b.Bildate
        ORDER BY b.Bildate ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_habit_history(pool: &DbPool, user_id: i32, habit_id: i32) -> Result<Vec<HabitHistory>, sqlx::Error> {
    sqlx::query_as::<_, HabitHistory>(
        r#"
        SELECT b.Bildate AS bildate, hb.HBdone AS hbdone
        FROM BILAN b
        JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid
        WHERE b.Usrid = ?
          AND hb.Habid = ?
        ORDER BY b.Bildate DESC
        "#,
    )
    .bind(user_id)
    .bind(habit_id)
    .fetch_all(pool)
    .await
}

pub async fn get_habit_completion_rate(pool: &DbPool, user_id: i32, habit_id: i32) -> Result<HabitCompletionRate, sqlx::Error> {
    sqlx::query_as::<_, HabitCompletionRate>(
        r#"
        SELECT
            COALESCE(AVG(hb.HBdone) * 100, 0) AS completion_rate
        FROM BILAN b
        JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid
        WHERE b.Usrid = ?
          AND hb.Habid = ?
        "#,
    )
    .bind(user_id)
    .bind(habit_id)
    .fetch_one(pool)
    .await
}

pub async fn get_most_consistent_habits(
    pool: &DbPool,
    user_id: i32,
    limit: i32,
) -> Result<Vec<HabitConsistency>, sqlx::Error> {
    sqlx::query_as::<_, HabitConsistency>(
        r#"
        SELECT h.Habid AS habid, h.Habnom AS habnom,
               COALESCE(AVG(hb.HBdone) * 100, 0) AS consistency_rate
        FROM HABITUDE h
        LEFT JOIN HABITUDE_BILAN hb ON hb.Habid = h.Habid
        WHERE h.Usrid = ?
        GROUP BY h.Habid, h.Habnom
        ORDER BY consistency_rate DESC
        LIMIT ?
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn get_least_consistent_habits(
    pool: &DbPool,
    user_id: i32,
    limit: i32,
) -> Result<Vec<HabitConsistency>, sqlx::Error> {
    sqlx::query_as::<_, HabitConsistency>(
        r#"
        SELECT h.Habid AS habid, h.Habnom AS habnom,
               COALESCE(AVG(hb.HBdone) * 100, 0) AS consistency_rate
        FROM HABITUDE h
        LEFT JOIN HABITUDE_BILAN hb ON hb.Habid = h.Habid
        WHERE h.Usrid = ?
        GROUP BY h.Habid, h.Habnom
        ORDER BY consistency_rate ASC
        LIMIT ?
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

// ===== SOBRIETY FUNCTIONS =====

pub async fn get_current_sobriety_period(pool: &DbPool, user_id: i32) -> Result<Option<SobrietyPeriod>, sqlx::Error> {
    sqlx::query_as::<_, SobrietyPeriod>(
        r#"
        SELECT s.Sobid AS sobid, s.Usrid AS usrid, s.Sobdebut AS sobdebut, s.Sobfin AS sobfin
        FROM SOBRIETE s
        WHERE s.Usrid = ?
          AND s.Sobfin IS NULL
        ORDER BY s.Sobdebut DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_current_sobriety_duration(pool: &DbPool, user_id: i32) -> Result<Option<SobrietyDuration>, sqlx::Error> {
    sqlx::query_as::<_, SobrietyDuration>(
        r#"
        SELECT TIMESTAMPDIFF(HOUR, s.Sobdebut, NOW()) AS sobriety_hours
        FROM SOBRIETE s
        WHERE s.Usrid = ?
          AND s.Sobfin IS NULL
        ORDER BY s.Sobdebut DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_sobriety_history(pool: &DbPool, user_id: i32) -> Result<Vec<SobrietyPeriod>, sqlx::Error> {
    sqlx::query_as::<_, SobrietyPeriod>(
        r#"
        SELECT s.Sobid AS sobid, s.Usrid AS usrid, s.Sobdebut AS sobdebut, s.Sobfin AS sobfin
        FROM SOBRIETE s
        WHERE s.Usrid = ?
        ORDER BY s.Sobdebut DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_total_sobriety_duration(pool: &DbPool, user_id: i32) -> Result<TotalSobrietyDuration, sqlx::Error> {
    sqlx::query_as::<_, TotalSobrietyDuration>(
        r#"
        SELECT COALESCE(SUM(TIMESTAMPDIFF(HOUR, s.Sobdebut, COALESCE(s.Sobfin, NOW()))), 0) AS total_sobriety_hours
        FROM SOBRIETE s
        WHERE s.Usrid = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_sobriety_stats_by_period(
    pool: &DbPool,
    user_id: i32,
    days: i32,
) -> Result<SobrietyStats, sqlx::Error> {
    sqlx::query_as::<_, SobrietyStats>(
        r#"
        SELECT
            COUNT(*) AS sobriety_period_count,
            COALESCE(SUM(TIMESTAMPDIFF(HOUR, s.Sobdebut, COALESCE(s.Sobfin, NOW()))), 0) AS sobriety_hours
        FROM SOBRIETE s
        WHERE s.Usrid = ?
          AND s.Sobdebut >= DATE_SUB(NOW(), INTERVAL ? DAY)
        "#,
    )
    .bind(user_id)
    .bind(days)
    .fetch_one(pool)
    .await
}

// ===== MOOD FUNCTIONS =====

pub async fn get_mood_types(pool: &DbPool) -> Result<Vec<MoodType>, sqlx::Error> {
    sqlx::query_as::<_, MoodType>(
        r#"
        SELECT h.Humid AS humid, h.Humnom AS humnom, h.Humcolor AS humcolor
        FROM HUMEUR h
        ORDER BY h.Humnom ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_today_mood(pool: &DbPool, user_id: i32) -> Result<Option<MoodEntry>, sqlx::Error> {
    sqlx::query_as::<_, MoodEntry>(
        r#"
        SELECT dh.Usrid AS usrid, dh.Humid AS humid, dh.DHdate AS dhdate
        FROM DATE_HUMEUR dh
        WHERE dh.Usrid = ?
          AND dh.DHdate = CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_mood_by_date(pool: &DbPool, user_id: i32, date: NaiveDate) -> Result<Option<MoodEntry>, sqlx::Error> {
    sqlx::query_as::<_, MoodEntry>(
        r#"
        SELECT dh.Usrid AS usrid, dh.Humid AS humid, dh.DHdate AS dhdate
        FROM DATE_HUMEUR dh
        WHERE dh.Usrid = ?
          AND dh.DHdate = ?
        "#,
    )
    .bind(user_id)
    .bind(date)
    .fetch_optional(pool)
    .await
}

pub async fn get_monthly_moods(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<MoodEntry>, sqlx::Error> {
    sqlx::query_as::<_, MoodEntry>(
        r#"
        SELECT dh.Usrid AS usrid, dh.Humid AS humid, dh.DHdate AS dhdate
        FROM DATE_HUMEUR dh
        WHERE dh.Usrid = ?
          AND dh.DHdate BETWEEN ? AND ?
        ORDER BY dh.DHdate ASC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

pub async fn get_most_frequent_mood(pool: &DbPool, user_id: i32) -> Result<Option<MostFrequentMood>, sqlx::Error> {
    sqlx::query_as::<_, MostFrequentMood>(
        r#"
        SELECT dh.Humid AS humid, h.Humnom AS humnom, COUNT(*) AS mood_count
        FROM DATE_HUMEUR dh
        JOIN HUMEUR h ON h.Humid = dh.Humid
        WHERE dh.Usrid = ?
        GROUP BY dh.Humid, h.Humnom
        ORDER BY mood_count DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_mood_distribution_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<MoodDistribution>, sqlx::Error> {
    sqlx::query_as::<_, MoodDistribution>(
        r#"
        SELECT h.Humid AS humid, h.Humnom AS humnom, COUNT(*) AS mood_count
        FROM DATE_HUMEUR dh
        JOIN HUMEUR h ON h.Humid = dh.Humid
        WHERE dh.Usrid = ?
          AND dh.DHdate BETWEEN ? AND ?
        GROUP BY h.Humid, h.Humnom
        ORDER BY mood_count DESC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

// ===== HYDRATION FUNCTIONS =====

pub async fn get_today_hydration(pool: &DbPool, user_id: i32) -> Result<Vec<HydrationEntry>, sqlx::Error> {
    sqlx::query_as::<_, HydrationEntry>(
        r#"
        SELECT h.Hydid AS hydid, h.Usrid AS usrid, h.Hyddate AS hyddate,
               h.Hydquantite AS hydquantite, h.Hydobjectif AS hydobjectif
        FROM HYDRATATION h
        WHERE h.Usrid = ?
          AND h.Hyddate = CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_today_hydration_goal(pool: &DbPool, user_id: i32) -> Result<Option<HydrationEntry>, sqlx::Error> {
    sqlx::query_as::<_, HydrationEntry>(
        r#"
        SELECT h.Hydid AS hydid, h.Usrid AS usrid, h.Hyddate AS hyddate,
               h.Hydquantite AS hydquantite, h.Hydobjectif AS hydobjectif
        FROM HYDRATATION h
        WHERE h.Usrid = ?
          AND h.Hyddate = CURDATE()
        ORDER BY h.Hydid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_hydration_history(pool: &DbPool, user_id: i32) -> Result<Vec<HydrationEntry>, sqlx::Error> {
    sqlx::query_as::<_, HydrationEntry>(
        r#"
        SELECT h.Hydid AS hydid, h.Usrid AS usrid, h.Hyddate AS hyddate,
               h.Hydquantite AS hydquantite, h.Hydobjectif AS hydobjectif
        FROM HYDRATATION h
        WHERE h.Usrid = ?
        ORDER BY h.Hyddate DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_today_water_total(pool: &DbPool, user_id: i32) -> Result<WaterTotal, sqlx::Error> {
    sqlx::query_as::<_, WaterTotal>(
        r#"
        SELECT COALESCE(SUM(h.Hydquantite), 0) AS water_total
        FROM HYDRATATION h
        WHERE h.Usrid = ?
          AND h.Hyddate = CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_hydration_goal_progress(pool: &DbPool, user_id: i32) -> Result<HydrationGoalProgress, sqlx::Error> {
    sqlx::query_as::<_, HydrationGoalProgress>(
        r#"
        SELECT
            CASE
                WHEN goal.goal_value = 0 THEN 0
                ELSE ROUND((today.today_value / goal.goal_value) * 100, 2)
            END AS goal_progress_percent
        FROM (
            SELECT COALESCE(SUM(h.Hydquantite), 0) AS today_value
            FROM HYDRATATION h
            WHERE h.Usrid = ? AND h.Hyddate = CURDATE()
        ) AS today
        CROSS JOIN (
            SELECT COALESCE(MAX(h.Hydobjectif), 0) AS goal_value
            FROM HYDRATATION h
            WHERE h.Usrid = ? AND h.Hyddate = CURDATE()
        ) AS goal
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_hydration_goal_reached_days(pool: &DbPool, user_id: i32) -> Result<Vec<HydrationGoalHistory>, sqlx::Error> {
    sqlx::query_as::<_, HydrationGoalHistory>(
        r#"
        SELECT h.Hyddate AS hyddate, h.Hydobjectif AS hydobjectif
        FROM HYDRATATION h
        WHERE h.Usrid = ?
          AND h.Hydquantite >= h.Hydobjectif
        ORDER BY h.Hyddate DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_hydration_goal_missed_days(pool: &DbPool, user_id: i32) -> Result<Vec<HydrationGoalHistory>, sqlx::Error> {
    sqlx::query_as::<_, HydrationGoalHistory>(
        r#"
        SELECT h.Hyddate AS hyddate, h.Hydobjectif AS hydobjectif
        FROM HYDRATATION h
        WHERE h.Usrid = ?
          AND h.Hydquantite < h.Hydobjectif
        ORDER BY h.Hyddate DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_weekly_hydration_average(pool: &DbPool, user_id: i32) -> Result<AverageWater, sqlx::Error> {
    sqlx::query_as::<_, AverageWater>(
        r#"
        SELECT ROUND(AVG(day_total), 2) AS weekly_average
        FROM (
            SELECT h.Hyddate, SUM(h.Hydquantite) AS day_total
            FROM HYDRATATION h
            WHERE h.Usrid = ?
              AND h.Hyddate BETWEEN DATE_SUB(CURDATE(), INTERVAL 6 DAY) AND CURDATE()
            GROUP BY h.Hyddate
        ) AS daily
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_monthly_hydration_average(pool: &DbPool, user_id: i32) -> Result<AverageWater, sqlx::Error> {
    sqlx::query_as::<_, AverageWater>(
        r#"
        SELECT ROUND(AVG(day_total), 2) AS weekly_average
        FROM (
            SELECT h.Hyddate, SUM(h.Hydquantite) AS day_total
            FROM HYDRATATION h
            WHERE h.Usrid = ?
              AND h.Hyddate BETWEEN DATE_SUB(CURDATE(), INTERVAL 29 DAY) AND CURDATE()
            GROUP BY h.Hyddate
        ) AS daily
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_hydration_goal_history(pool: &DbPool, user_id: i32) -> Result<Vec<HydrationGoalHistory>, sqlx::Error> {
    sqlx::query_as::<_, HydrationGoalHistory>(
        r#"
        SELECT h.Hyddate AS hyddate, h.Hydobjectif AS hydobjectif
        FROM HYDRATATION h
        WHERE h.Usrid = ?
        ORDER BY h.Hyddate ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

// ===== SLEEP FUNCTIONS =====

pub async fn get_latest_sleep_entry(pool: &DbPool, user_id: i32) -> Result<Option<SleepEntry>, sqlx::Error> {
    sqlx::query_as::<_, SleepEntry>(
        r#"
        SELECT s.Somid AS somid, s.Usrid AS usrid, s.Somdate AS somdate,
               s.Somcoucher AS somcoucher, s.Somlever AS somlever,
               s.Somduree AS somduree, s.Somreposant AS somreposant
        FROM SOMMEIL s
        WHERE s.Usrid = ?
        ORDER BY s.Somdate DESC, s.Somid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_today_sleep(pool: &DbPool, user_id: i32) -> Result<Option<SleepEntry>, sqlx::Error> {
    sqlx::query_as::<_, SleepEntry>(
        r#"
        SELECT s.Somid AS somid, s.Usrid AS usrid, s.Somdate AS somdate,
               s.Somcoucher AS somcoucher, s.Somlever AS somlever,
               s.Somduree AS somduree, s.Somreposant AS somreposant
        FROM SOMMEIL s
        WHERE s.Usrid = ?
          AND s.Somdate = CURDATE()
        ORDER BY s.Somid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_sleep_history(pool: &DbPool, user_id: i32) -> Result<Vec<SleepEntry>, sqlx::Error> {
    sqlx::query_as::<_, SleepEntry>(
        r#"
        SELECT s.Somid AS somid, s.Usrid AS usrid, s.Somdate AS somdate,
               s.Somcoucher AS somcoucher, s.Somlever AS somlever,
               s.Somduree AS somduree, s.Somreposant AS somreposant
        FROM SOMMEIL s
        WHERE s.Usrid = ?
        ORDER BY s.Somdate DESC, s.Somid DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_restful_sleep_entries(pool: &DbPool, user_id: i32) -> Result<Vec<SleepEntry>, sqlx::Error> {
    sqlx::query_as::<_, SleepEntry>(
        r#"
        SELECT s.Somid AS somid, s.Usrid AS usrid, s.Somdate AS somdate,
               s.Somcoucher AS somcoucher, s.Somlever AS somlever,
               s.Somduree AS somduree, s.Somreposant AS somreposant
        FROM SOMMEIL s
        WHERE s.Usrid = ?
          AND s.Somreposant = 1
        ORDER BY s.Somdate DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_non_restful_sleep_entries(pool: &DbPool, user_id: i32) -> Result<Vec<SleepEntry>, sqlx::Error> {
    sqlx::query_as::<_, SleepEntry>(
        r#"
        SELECT s.Somid AS somid, s.Usrid AS usrid, s.Somdate AS somdate,
               s.Somcoucher AS somcoucher, s.Somlever AS somlever,
               s.Somduree AS somduree, s.Somreposant AS somreposant
        FROM SOMMEIL s
        WHERE s.Usrid = ?
          AND s.Somreposant = 0
        ORDER BY s.Somdate DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_weekly_sleep_average(pool: &DbPool, user_id: i32) -> Result<AverageSleep, sqlx::Error> {
    sqlx::query_as::<_, AverageSleep>(
        r#"
        SELECT ROUND(AVG(s.Somduree), 2) AS weekly_sleep_average
        FROM SOMMEIL s
        WHERE s.Usrid = ?
          AND s.Somdate BETWEEN DATE_SUB(CURDATE(), INTERVAL 6 DAY) AND CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_monthly_sleep_average(pool: &DbPool, user_id: i32) -> Result<AverageSleep, sqlx::Error> {
    sqlx::query_as::<_, AverageSleep>(
        r#"
        SELECT ROUND(AVG(s.Somduree), 2) AS weekly_sleep_average
        FROM SOMMEIL s
        WHERE s.Usrid = ?
          AND s.Somdate BETWEEN DATE_SUB(CURDATE(), INTERVAL 29 DAY) AND CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_short_sleep_entries(pool: &DbPool, user_id: i32, min_duration: i32) -> Result<Vec<SleepEntry>, sqlx::Error> {
    sqlx::query_as::<_, SleepEntry>(
        r#"
        SELECT s.Somid AS somid, s.Usrid AS usrid, s.Somdate AS somdate,
               s.Somcoucher AS somcoucher, s.Somlever AS somlever,
               s.Somduree AS somduree, s.Somreposant AS somreposant
        FROM SOMMEIL s
        WHERE s.Usrid = ?
          AND COALESCE(s.Somduree, TIMESTAMPDIFF(MINUTE, s.Somcoucher, s.Somlever)) < ?
        ORDER BY s.Somdate DESC
        "#,
    )
    .bind(user_id)
    .bind(min_duration)
    .fetch_all(pool)
    .await
}

// ===== NUTRITION FUNCTIONS =====

pub async fn get_today_meals(pool: &DbPool, user_id: i32) -> Result<Vec<Meal>, sqlx::Error> {
    sqlx::query_as::<_, Meal>(
        r#"
        SELECT r.Repid AS repid, r.Usrid AS usrid, r.Repdate AS repdate,
               r.Repcalories AS repcalories, r.Repproteines AS repproteines,
               r.Repglucides AS repglucides, r.Replipides AS replipides
        FROM REPAS r
        WHERE r.Usrid = ?
          AND r.Repdate = CURDATE()
        ORDER BY r.Repid DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_meals_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<Meal>, sqlx::Error> {
    sqlx::query_as::<_, Meal>(
        r#"
        SELECT r.Repid AS repid, r.Usrid AS usrid, r.Repdate AS repdate,
               r.Repcalories AS repcalories, r.Repproteines AS repproteines,
               r.Repglucides AS repglucides, r.Replipides AS replipides
        FROM REPAS r
        WHERE r.Usrid = ?
          AND r.Repdate BETWEEN ? AND ?
        ORDER BY r.Repdate ASC, r.Repid ASC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

pub async fn get_latest_meal(pool: &DbPool, user_id: i32) -> Result<Option<Meal>, sqlx::Error> {
    sqlx::query_as::<_, Meal>(
        r#"
        SELECT r.Repid AS repid, r.Usrid AS usrid, r.Repdate AS repdate,
               r.Repcalories AS repcalories, r.Repproteines AS repproteines,
               r.Repglucides AS repglucides, r.Replipides AS replipides
        FROM REPAS r
        WHERE r.Usrid = ?
        ORDER BY r.Repdate DESC, r.Repid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_today_calorie_total(pool: &DbPool, user_id: i32) -> Result<CalorieTotal, sqlx::Error> {
    sqlx::query_as::<_, CalorieTotal>(
        r#"
        SELECT COALESCE(SUM(r.Repcalories), 0) AS calorie_total
        FROM REPAS r
        WHERE r.Usrid = ?
          AND r.Repdate = CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_today_protein_total(pool: &DbPool, user_id: i32) -> Result<ProteinTotal, sqlx::Error> {
    sqlx::query_as::<_, ProteinTotal>(
        r#"
        SELECT COALESCE(SUM(r.Repproteines), 0) AS protein_total
        FROM REPAS r
        WHERE r.Usrid = ?
          AND r.Repdate = CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_today_carb_total(pool: &DbPool, user_id: i32) -> Result<CalorieTotal, sqlx::Error> {
    sqlx::query_as::<_, CalorieTotal>(
        r#"
        SELECT COALESCE(SUM(r.Repglucides), 0) AS calorie_total
        FROM REPAS r
        WHERE r.Usrid = ?
          AND r.Repdate = CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_today_fat_total(pool: &DbPool, user_id: i32) -> Result<CalorieTotal, sqlx::Error> {
    sqlx::query_as::<_, CalorieTotal>(
        r#"
        SELECT COALESCE(SUM(r.Replipides), 0) AS calorie_total
        FROM REPAS r
        WHERE r.Usrid = ?
          AND r.Repdate = CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_daily_macro_distribution(pool: &DbPool, user_id: i32) -> Result<MacroDistribution, sqlx::Error> {
    sqlx::query_as::<_, MacroDistribution>(
        r#"
        SELECT
            COALESCE(SUM(r.Repproteines), 0) AS proteins,
            COALESCE(SUM(r.Repglucides), 0) AS carbs,
            COALESCE(SUM(r.Replipides), 0) AS fats
        FROM REPAS r
        WHERE r.Usrid = ?
          AND r.Repdate = CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_nutrition_history(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<NutritionHistory>, sqlx::Error> {
    sqlx::query_as::<_, NutritionHistory>(
        r#"
        SELECT r.Repdate AS repdate,
               COALESCE(SUM(r.Repcalories), 0) AS calories,
               COALESCE(SUM(r.Repproteines), 0) AS proteins,
               COALESCE(SUM(r.Repglucides), 0) AS carbs,
               COALESCE(SUM(r.Replipides), 0) AS fats
        FROM REPAS r
        WHERE r.Usrid = ?
          AND r.Repdate BETWEEN ? AND ?
        GROUP BY r.Repdate
        ORDER BY r.Repdate ASC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

pub async fn get_weekly_calorie_average(pool: &DbPool, user_id: i32) -> Result<AverageCalories, sqlx::Error> {
    sqlx::query_as::<_, AverageCalories>(
        r#"
        SELECT ROUND(AVG(day_calories), 2) AS weekly_calorie_average
        FROM (
            SELECT r.Repdate, COALESCE(SUM(r.Repcalories), 0) AS day_calories
            FROM REPAS r
            WHERE r.Usrid = ?
              AND r.Repdate BETWEEN DATE_SUB(CURDATE(), INTERVAL 6 DAY) AND CURDATE()
            GROUP BY r.Repdate
        ) AS daily
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_monthly_calorie_average(pool: &DbPool, user_id: i32) -> Result<AverageCalories, sqlx::Error> {
    sqlx::query_as::<_, AverageCalories>(
        r#"
        SELECT ROUND(AVG(day_calories), 2) AS weekly_calorie_average
        FROM (
            SELECT r.Repdate, COALESCE(SUM(r.Repcalories), 0) AS day_calories
            FROM REPAS r
            WHERE r.Usrid = ?
              AND r.Repdate BETWEEN DATE_SUB(CURDATE(), INTERVAL 29 DAY) AND CURDATE()
            GROUP BY r.Repdate
        ) AS daily
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

// ===== BODY MEASUREMENT FUNCTIONS =====

pub async fn get_latest_body_measurement(pool: &DbPool, user_id: i32) -> Result<Option<BodyMeasurement>, sqlx::Error> {
    sqlx::query_as::<_, BodyMeasurement>(
        r#"
        SELECT m.Mesid AS mesid, m.Usrid AS usrid, m.Mesdate AS mesdate,
               m.Mespoids AS mespoids, m.Mestaille AS mestaille,
               m.MesIMC AS mesIMC, m.MesMetaBasal AS mesMetaBasal
        FROM MESURE_CORPORELLE m
        WHERE m.Usrid = ?
        ORDER BY m.Mesdate DESC, m.Mesid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_weight_history(pool: &DbPool, user_id: i32) -> Result<Vec<WeightEntry>, sqlx::Error> {
    sqlx::query_as::<_, WeightEntry>(
        r#"
        SELECT m.Mesdate AS mesdate, m.Mespoids AS mespoids
        FROM MESURE_CORPORELLE m
        WHERE m.Usrid = ?
        ORDER BY m.Mesdate ASC, m.Mesid ASC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_weight_chart_data(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<WeightEntry>, sqlx::Error> {
    sqlx::query_as::<_, WeightEntry>(
        r#"
        SELECT m.Mesdate AS mesdate, m.Mespoids AS mespoids
        FROM MESURE_CORPORELLE m
        WHERE m.Usrid = ?
          AND m.Mesdate BETWEEN ? AND ?
        ORDER BY m.Mesdate ASC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

pub async fn get_weight_progress(pool: &DbPool, user_id: i32) -> Result<Option<WeightProgress>, sqlx::Error> {
    sqlx::query_as::<_, WeightProgress>(
        r#"
        SELECT
            first_weight,
            last_weight,
            last_weight - first_weight AS weight_delta
        FROM (
            SELECT
                (SELECT m.Mespoids FROM MESURE_CORPORELLE m WHERE m.Usrid = ? ORDER BY m.Mesdate ASC, m.Mesid ASC LIMIT 1) AS first_weight,
                (SELECT m.Mespoids FROM MESURE_CORPORELLE m WHERE m.Usrid = ? ORDER BY m.Mesdate DESC, m.Mesid DESC LIMIT 1) AS last_weight
        ) AS weights
        "#,
    )
    .bind(user_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_current_bmi(pool: &DbPool, user_id: i32) -> Result<Option<CurrentBmi>, sqlx::Error> {
    sqlx::query_as::<_, CurrentBmi>(
        r#"
        SELECT m.MesIMC AS mesIMC
        FROM MESURE_CORPORELLE m
        WHERE m.Usrid = ?
        ORDER BY m.Mesdate DESC, m.Mesid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_health_derived_metrics(pool: &DbPool, user_id: i32) -> Result<Option<HealthMetrics>, sqlx::Error> {
    sqlx::query_as::<_, HealthMetrics>(
        r#"
        SELECT
            latest.Mespoids AS latest_weight,
            latest.Mestaille AS latest_height,
            latest.MesIMC AS latest_bmi,
            latest.MesMetaBasal AS latest_basal_metabolism
        FROM MESURE_CORPORELLE latest
        WHERE latest.Usrid = ?
        ORDER BY latest.Mesdate DESC, latest.Mesid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

// ===== SPORT FUNCTIONS =====

pub async fn get_sport_types(pool: &DbPool) -> Result<Vec<SportType>, sqlx::Error> {
    sqlx::query_as::<_, SportType>(
        r#"
        SELECT s.Stypid AS stypid, s.Stypnom AS stypnom
        FROM SPORT_TYPE s
        ORDER BY s.Stypnom ASC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_today_sport_sessions(pool: &DbPool, user_id: i32) -> Result<Vec<SportSession>, sqlx::Error> {
    sqlx::query_as::<_, SportSession>(
        r#"
        SELECT s.Seaid AS seaid, s.Usrid AS usrid, s.Stypid AS stypid, s.Seadate AS seadate,
               s.Seaduree AS seaduree, s.Seacalories AS seacalories, s.Seaintensite AS seaintensite
        FROM SEANCE_SPORT s
        WHERE s.Usrid = ?
          AND s.Seadate = CURDATE()
        ORDER BY s.Seaid DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_sport_sessions_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<SportSession>, sqlx::Error> {
    sqlx::query_as::<_, SportSession>(
        r#"
        SELECT s.Seaid AS seaid, s.Usrid AS usrid, s.Stypid AS stypid, s.Seadate AS seadate,
               s.Seaduree AS seaduree, s.Seacalories AS seacalories, s.Seaintensite AS seaintensite
        FROM SEANCE_SPORT s
        WHERE s.Usrid = ?
          AND s.Seadate BETWEEN ? AND ?
        ORDER BY s.Seadate ASC, s.Seaid ASC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

pub async fn get_latest_sport_session(pool: &DbPool, user_id: i32) -> Result<Option<SportSession>, sqlx::Error> {
    sqlx::query_as::<_, SportSession>(
        r#"
        SELECT s.Seaid AS seaid, s.Usrid AS usrid, s.Stypid AS stypid, s.Seadate AS seadate,
               s.Seaduree AS seaduree, s.Seacalories AS seacalories, s.Seaintensite AS seaintensite
        FROM SEANCE_SPORT s
        WHERE s.Usrid = ?
        ORDER BY s.Seadate DESC, s.Seaid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_total_sport_duration_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<SportDurationTotal, sqlx::Error> {
    sqlx::query_as::<_, SportDurationTotal>(
        r#"
        SELECT COALESCE(SUM(s.Seaduree), 0) AS sport_duration_total
        FROM SEANCE_SPORT s
        WHERE s.Usrid = ?
          AND s.Seadate BETWEEN ? AND ?
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await
}

// ===== SPORT FUNCTIONS =====

pub async fn get_sport_session_count_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<SportSessionCount, sqlx::Error> {
    sqlx::query_as::<_, SportSessionCount>(
        r#"
        SELECT COUNT(*) AS sport_session_count
        FROM SEANCE_SPORT s
        WHERE s.Usrid = ?
          AND s.Seadate BETWEEN ? AND ?
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await
}

pub async fn get_burned_calories_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<BurnedCalories, sqlx::Error> {
    sqlx::query_as::<_, BurnedCalories>(
        r#"
        SELECT COALESCE(SUM(s.Seacalories), 0) AS burned_calories
        FROM SEANCE_SPORT s
        WHERE s.Usrid = ?
          AND s.Seadate BETWEEN ? AND ?
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await
}

pub async fn get_sport_sessions_by_type(
    pool: &DbPool,
    user_id: i32,
    type_id: i32,
) -> Result<Vec<SportSession>, sqlx::Error> {
    sqlx::query_as::<_, SportSession>(
        r#"
        SELECT s.Seaid AS seaid, s.Usrid AS usrid, s.Stypid AS stypid, s.Seadate AS seadate,
               s.Seaduree AS seaduree, s.Seacalories AS seacalories, s.Seaintensite AS seaintensite
        FROM SEANCE_SPORT s
        WHERE s.Usrid = ?
          AND s.Stypid = ?
        ORDER BY s.Seadate DESC, s.Seaid DESC
        "#,
    )
    .bind(user_id)
    .bind(type_id)
    .fetch_all(pool)
    .await
}

pub async fn get_most_practiced_sport(pool: &DbPool, user_id: i32) -> Result<Option<MostPracticedSport>, sqlx::Error> {
    sqlx::query_as::<_, MostPracticedSport>(
        r#"
        SELECT st.Stypid AS stypid, st.Stypnom AS stypnom, COUNT(*) AS session_count
        FROM SEANCE_SPORT ss
        LEFT JOIN SPORT_TYPE st ON st.Stypid = ss.Stypid
        WHERE ss.Usrid = ?
        GROUP BY st.Stypid, st.Stypnom
        ORDER BY session_count DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_weekly_sport_stats(pool: &DbPool, user_id: i32) -> Result<SportStats, sqlx::Error> {
    sqlx::query_as::<_, SportStats>(
        r#"
        SELECT
            COUNT(*) AS session_count,
            COALESCE(SUM(s.Seaduree), 0) AS total_duration,
            COALESCE(SUM(s.Seacalories), 0) AS total_calories
        FROM SEANCE_SPORT s
        WHERE s.Usrid = ?
          AND s.Seadate BETWEEN DATE_SUB(CURDATE(), INTERVAL 6 DAY) AND CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_monthly_sport_stats(pool: &DbPool, user_id: i32) -> Result<SportStats, sqlx::Error> {
    sqlx::query_as::<_, SportStats>(
        r#"
        SELECT
            COUNT(*) AS session_count,
            COALESCE(SUM(s.Seaduree), 0) AS total_duration,
            COALESCE(SUM(s.Seacalories), 0) AS total_calories
        FROM SEANCE_SPORT s
        WHERE s.Usrid = ?
          AND s.Seadate BETWEEN DATE_SUB(CURDATE(), INTERVAL 29 DAY) AND CURDATE()
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_sport_chart_data(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<SportChartData>, sqlx::Error> {
    sqlx::query_as::<_, SportChartData>(
        r#"
        SELECT s.Seadate AS seadate,
               COALESCE(SUM(s.Seaduree), 0) AS duration,
               COALESCE(SUM(s.Seacalories), 0) AS calories
        FROM SEANCE_SPORT s
        WHERE s.Usrid = ?
          AND s.Seadate BETWEEN ? AND ?
        GROUP BY s.Seadate
        ORDER BY s.Seadate ASC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

// ===== BREATHING/COHERENCE FUNCTIONS =====

pub async fn get_latest_breathing_session(pool: &DbPool, user_id: i32) -> Result<Option<BreathingSession>, sqlx::Error> {
    sqlx::query_as::<_, BreathingSession>(
        r#"
        SELECT c.Cohid AS cohid, c.Usrid AS usrid, c.Cohdateheure AS cohdateheure, c.Cohduree AS cohduree
        FROM COHERENCE_CARDIAQUE c
        WHERE c.Usrid = ?
        ORDER BY c.Cohdateheure DESC, c.Cohid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_today_breathing_sessions(pool: &DbPool, user_id: i32) -> Result<Vec<BreathingSession>, sqlx::Error> {
    sqlx::query_as::<_, BreathingSession>(
        r#"
        SELECT c.Cohid AS cohid, c.Usrid AS usrid, c.Cohdateheure AS cohdateheure, c.Cohduree AS cohduree
        FROM COHERENCE_CARDIAQUE c
        WHERE c.Usrid = ?
          AND DATE(c.Cohdateheure) = CURDATE()
        ORDER BY c.Cohdateheure DESC, c.Cohid DESC
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await
}

pub async fn get_breathing_sessions_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<BreathingSession>, sqlx::Error> {
    sqlx::query_as::<_, BreathingSession>(
        r#"
        SELECT c.Cohid AS cohid, c.Usrid AS usrid, c.Cohdateheure AS cohdateheure, c.Cohduree AS cohduree
        FROM COHERENCE_CARDIAQUE c
        WHERE c.Usrid = ?
          AND DATE(c.Cohdateheure) BETWEEN ? AND ?
        ORDER BY c.Cohdateheure ASC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

pub async fn get_total_breathing_duration_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<BreathingDurationTotal, sqlx::Error> {
    sqlx::query_as::<_, BreathingDurationTotal>(
        r#"
        SELECT COALESCE(SUM(c.Cohduree), 0) AS breathing_duration_total
        FROM COHERENCE_CARDIAQUE c
        WHERE c.Usrid = ?
          AND DATE(c.Cohdateheure) BETWEEN ? AND ?
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await
}

pub async fn get_breathing_session_count_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<BreathingSessionCount, sqlx::Error> {
    sqlx::query_as::<_, BreathingSessionCount>(
        r#"
        SELECT COUNT(*) AS breathing_session_count
        FROM COHERENCE_CARDIAQUE c
        WHERE c.Usrid = ?
          AND DATE(c.Cohdateheure) BETWEEN ? AND ?
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await
}

pub async fn get_average_breathing_usage_frequency(pool: &DbPool, user_id: i32) -> Result<BreathingFrequency, sqlx::Error> {
    sqlx::query_as::<_, BreathingFrequency>(
        r#"
        SELECT ROUND(COUNT(*) / NULLIF(COUNT(DISTINCT DATE(c.Cohdateheure)), 0), 2) AS average_frequency
        FROM COHERENCE_CARDIAQUE c
        WHERE c.Usrid = ?
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

// ===== ALCOHOL FUNCTIONS =====

pub async fn get_latest_alcohol_entry(pool: &DbPool, user_id: i32) -> Result<Option<AlcoholEntry>, sqlx::Error> {
    sqlx::query_as::<_, AlcoholEntry>(
        r#"
        SELECT c.Alcid AS alcid, c.Usrid AS usrid, c.Alcdateheure AS alcdateheure,
               c.Alcalcoolemie AS alcalcoolemie, c.Alctempsobre AS alctempsobre
        FROM CONSOMMATION_ALCOOL c
        WHERE c.Usrid = ?
        ORDER BY c.Alcdateheure DESC, c.Alcid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_alcohol_entries_by_period(
    pool: &DbPool,
    user_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<AlcoholEntry>, sqlx::Error> {
    sqlx::query_as::<_, AlcoholEntry>(
        r#"
        SELECT c.Alcid AS alcid, c.Usrid AS usrid, c.Alcdateheure AS alcdateheure,
               c.Alcalcoolemie AS alcalcoolemie, c.Alctempsobre AS alctempsobre
        FROM CONSOMMATION_ALCOOL c
        WHERE c.Usrid = ?
          AND DATE(c.Alcdateheure) BETWEEN ? AND ?
        ORDER BY c.Alcdateheure ASC, c.Alcid ASC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

pub async fn get_current_blood_alcohol_level(pool: &DbPool, user_id: i32) -> Result<Option<BloodAlcoholLevel>, sqlx::Error> {
    sqlx::query_as::<_, BloodAlcoholLevel>(
        r#"
        SELECT c.Alcalcoolemie AS alcalcoolemie
        FROM CONSOMMATION_ALCOOL c
        WHERE c.Usrid = ?
        ORDER BY c.Alcdateheure DESC, c.Alcid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_time_until_sobriety(pool: &DbPool, user_id: i32) -> Result<Option<TimeUntilSobriety>, sqlx::Error> {
    sqlx::query_as::<_, TimeUntilSobriety>(
        r#"
        SELECT c.Alctempsobre AS alctempsobre
        FROM CONSOMMATION_ALCOOL c
        WHERE c.Usrid = ?
        ORDER BY c.Alcdateheure DESC, c.Alcid DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

use chrono::NaiveDate;
use sqlx;
use crate::db::DbPool;

pub async fn create_user(pool: &DbPool, public_id: &str) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO UTILISATEUR (UsrpublicId) VALUES (?)"#)
        .bind(public_id)
        .execute(pool)
        .await?;
    Ok(result.last_insert_id() as i32)
}

pub async fn create_account(pool: &DbPool, user_id: i32, name: &str, balance: f64) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO COMPTE (Comnom, Comsolde, Usrid) VALUES (?, ?, ?)"#)
        .bind(name)
        .bind(balance)
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(result.last_insert_id() as i32)
}

pub async fn create_finance_type(pool: &DbPool, name: &str) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO TYPE (Typtitre) VALUES (?)"#)
        .bind(name)
        .execute(pool)
        .await?;
    Ok(result.last_insert_id() as i32)
}

pub async fn create_transaction(pool: &DbPool, user_id: i32, account_id: i32, type_id: i32, amount: f64, description: &str) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO MOUVEMENT (Moumontant, Moudate, Typid, Comid, Usrid) VALUES (?, CURDATE(), ?, ?, ?)"#)
        .bind(amount)
        .bind(type_id)
        .bind(account_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    let id = result.last_insert_id() as i32;
    let _ = description;
    Ok(id)
}

pub async fn create_planned_expense(
    pool: &DbPool,
    user_id: i32,
    description: &str,
    amount: f64,
    account_id: Option<i32>,
    type_id: Option<i32>,
    periodicite: &str,
    intervalle: i32,
    next_date: NaiveDate,
) -> Result<i32, sqlx::Error> {
    let mou = sqlx::query(
        r#"INSERT INTO MOUVEMENT (Moumontant, Moudate, Typid, Comid, Usrid, Moudescription)
           VALUES (?, CURDATE(), ?, ?, ?, ?)"#,
    )
    .bind(amount)
    .bind(type_id)
    .bind(account_id)
    .bind(user_id)
    .bind(description)
    .execute(pool)
    .await?;
    let mouid = mou.last_insert_id() as i32;

    let fac = sqlx::query(
        r#"INSERT INTO FACTURE (Facdate, Facperiodicite, Facintervalle, FacdateProchain, Facdone, Mouid, Typid, Comid, Usrid)
           VALUES (CURDATE(), ?, ?, ?, 0, ?, ?, ?, ?)"#,
    )
    .bind(periodicite)
    .bind(intervalle)
    .bind(next_date)
    .bind(mouid)
    .bind(type_id)
    .bind(account_id)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(fac.last_insert_id() as i32)
}
pub async fn create_habit_category(pool: &DbPool, name: &str) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO CATEGORIE (Catnom, Catplus) VALUES (?, '1')"#)
        .bind(name)
        .execute(pool)
        .await?;
    Ok(result.last_insert_id() as i32)
}

pub async fn create_habit(pool: &DbPool, user_id: i32, category_id: i32, title: &str, description: &str, habit_type: &str) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO HABITUDE (Habnom, Catid, Usrid) VALUES (?, ?, ?)"#)
        .bind(title)
        .bind(category_id)
        .bind(user_id)
        .execute(pool)
        .await?;
    let _ = (description, habit_type);
    Ok(result.last_insert_id() as i32)
}

pub async fn mark_habit_complete(pool: &DbPool, user_id: i32, habit_id: i32, date: NaiveDate) -> Result<i32, sqlx::Error> {
    let bilan = sqlx::query(r#"INSERT INTO BILAN (Bildate, Humid, Usrid) VALUES (?, NULL, ?)"#)
        .bind(date)
        .bind(user_id)
        .execute(pool)
        .await?;
    let bilid = bilan.last_insert_id() as i32;
    let result = sqlx::query(r#"INSERT INTO HABITUDE_BILAN (Bilid, Habid, HBdone) VALUES (?, ?, 1)"#)
        .bind(bilid)
        .bind(habit_id)
        .execute(pool)
        .await?;
    Ok(result.last_insert_id() as i32)
}

pub async fn create_sobriety_period(pool: &DbPool, user_id: i32, start_date: NaiveDate) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO SOBRIETE (Sobdebut, Usrid) VALUES (?, ?)"#)
        .bind(start_date.and_hms_opt(0, 0, 0).unwrap())
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(result.last_insert_id() as i32)
}

pub async fn create_mood_type(pool: &DbPool, name: &str) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO HUMEUR (Humnom, Humcolor) VALUES (?, '#808080')"#)
        .bind(name)
        .execute(pool)
        .await?;
    Ok(result.last_insert_id() as i32)
}

pub async fn log_mood(pool: &DbPool, user_id: i32, type_id: i32, date: NaiveDate, notes: Option<&str>) -> Result<(), sqlx::Error> {
    sqlx::query(r#"INSERT INTO DATE_HUMEUR (Usrid, DHdate, Humid) VALUES (?, ?, ?)"#)
        .bind(user_id)
        .bind(date)
        .bind(type_id)
        .execute(pool)
        .await?;
    let _ = notes;
    Ok(())
}

pub async fn log_hydration(pool: &DbPool, user_id: i32, date: NaiveDate, quantity: i32, hydration_type: &str, objective: i32) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO HYDRATATION (Hyddate, Hydquantite, Hydobjectif, Usrid) VALUES (?, ?, ?, ?)"#)
        .bind(date)
        .bind(quantity)
        .bind(objective)
        .bind(user_id)
        .execute(pool)
        .await?;
    let _ = hydration_type;
    Ok(result.last_insert_id() as i32)
}

pub async fn log_sleep(pool: &DbPool, user_id: i32, date: NaiveDate, time: &str, duration: i32, quality: f64, is_restful: bool) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO SOMMEIL (Somdate, Somcoucher, Somlever, Somduree, Somreposant, Usrid) VALUES (?, ?, ?, ?, ?, ?)"#)
        .bind(date)
        .bind(time)
        .bind(time)
        .bind(duration)
        .bind(is_restful as i32)
        .bind(user_id)
        .execute(pool)
        .await?;
    let _ = quality;
    Ok(result.last_insert_id() as i32)
}

pub async fn log_meal(pool: &DbPool, user_id: i32, date: NaiveDate, time: &str, name: &str, calories: f64, proteins: f64, carbs: f64, fats: f64) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO REPAS (Repdate, Repdescription, Repcalories, Repproteines, Repglucides, Replipides, Usrid) VALUES (?, ?, ?, ?, ?, ?, ?)"#)
        .bind(date)
        .bind(name)
        .bind(calories)
        .bind(proteins)
        .bind(carbs)
        .bind(fats)
        .bind(user_id)
        .execute(pool)
        .await?;
    let _ = time;
    Ok(result.last_insert_id() as i32)
}

pub async fn log_body_measurement(pool: &DbPool, user_id: i32, date: NaiveDate, weight: f64, height: f64, chest: f64, waist: f64, hips: f64) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO MESURE_CORPORELLE (Mesdate, Mespoids, Mestaille, MesIMC, MesMetaBasal, Usrid) VALUES (?, ?, ?, NULL, NULL, ?)"#)
        .bind(date)
        .bind(weight)
        .bind(height)
        .bind(user_id)
        .execute(pool)
        .await?;
    let _ = (chest, waist, hips);
    Ok(result.last_insert_id() as i32)
}

pub async fn create_sport_type(pool: &DbPool, name: &str) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO SPORT_TYPE (Stypnom) VALUES (?)"#)
        .bind(name)
        .execute(pool)
        .await?;
    Ok(result.last_insert_id() as i32)
}

pub async fn log_sport_session(pool: &DbPool, user_id: i32, type_id: i32, date: NaiveDate, time: &str, duration: i32, calories: f64, intensity: &str) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO SEANCE_SPORT (Seadate, Stypid, Seaduree, Seaintensite, Seacalories, Usrid) VALUES (?, ?, ?, ?, ?, ?)"#)
        .bind(date)
        .bind(type_id)
        .bind(duration)
        .bind(intensity)
        .bind(calories)
        .bind(user_id)
        .execute(pool)
        .await?;
    let _ = time;
    Ok(result.last_insert_id() as i32)
}

pub async fn log_breathing_session(pool: &DbPool, user_id: i32, date: NaiveDate, time: &str, duration: i32, frequency: &str) -> Result<i32, sqlx::Error> {
    let dt = date.and_hms_opt(0, 0, 0).unwrap();
    let result = sqlx::query(r#"INSERT INTO COHERENCE_CARDIAQUE (Cohdateheure, Cohduree, Cohparamcercle, Usrid) VALUES (?, ?, NULL, ?)"#)
        .bind(dt)
        .bind(duration)
        .bind(user_id)
        .execute(pool)
        .await?;
    let _ = (time, frequency);
    Ok(result.last_insert_id() as i32)
}

pub async fn log_alcohol_consumption(pool: &DbPool, user_id: i32, date: NaiveDate, time: &str, alcohol_type: &str, quantity: f64, percentage: f64) -> Result<i32, sqlx::Error> {
    let dt = date.and_hms_opt(0, 0, 0).unwrap();
    let result = sqlx::query(r#"INSERT INTO CONSOMMATION_ALCOOL (Alcdateheure, Alcquantite, Alcdegre, Alcjeun, Usrid) VALUES (?, ?, ?, 0, ?)"#)
        .bind(dt)
        .bind(quantity)
        .bind(percentage)
        .bind(user_id)
        .execute(pool)
        .await?;
    let _ = (time, alcohol_type);
    Ok(result.last_insert_id() as i32)
}

pub async fn create_todo(pool: &DbPool, user_id: i32, title: &str, description: Option<&str>, due_date: Option<NaiveDate>) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(r#"INSERT INTO TODO (Todtitre, Toddone, Todtimer, Totypid, Usrid) VALUES (?, 0, NULL, NULL, ?)"#)
        .bind(title)
        .bind(user_id)
        .execute(pool)
        .await?;
    let _ = (description, due_date);
    Ok(result.last_insert_id() as i32)
}
