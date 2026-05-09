--
-- -----------------------------------------------------------
-- REQUÊTES GLOBALES
-- -----------------------------------------------------------

-- getUserProfile
-- paramètre: user_id
SELECT
		u.Usrid,
		u.UsrpublicId,
		u.UsrcreatedAt
FROM UTILISATEUR u
WHERE u.Usrid = ?;

-- getUserDisplaySettings
-- TODO: nécessite une table de paramètres utilisateur absente du schéma.
SELECT NULL AS setting_key, NULL AS setting_value
WHERE 1 = 0;

-- getActivityTips
-- TODO: nécessite une table de conseils/contenus ou des données statiques pré-chargées.
SELECT NULL AS activity_key, NULL AS tip_text
WHERE 1 = 0;

-- getTodayDashboard
-- paramètres: user_id répété dans le même ordre que les sous-requêtes ci-dessous
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
		 WHERE s.Usrid = ? AND s.Somdate = CURDATE()) AS today_sleep_count;

-- getModuleDataByPeriod
-- modèle générique; remplacez la table et la colonne de date pour le module ciblé.
-- paramètres: user_id, start_date, end_date
SELECT *
FROM REPAS
WHERE Usrid = ?
	AND Repdate BETWEEN ? AND ?
ORDER BY Repdate ASC;

-- getLatestModuleValues
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
SELECT 'MESURE_CORPORELLE', 'weight', CAST(m.Mespoids AS CHAR), CAST(m.Mesdate AS CHAR)
FROM MESURE_CORPORELLE m
WHERE m.Usrid = ?
ORDER BY m.Mesdate DESC, m.Mesid DESC
LIMIT 1
UNION ALL
SELECT 'SEANCE_SPORT', 'calories', CAST(s.Seacalories AS CHAR), CAST(s.Seadate AS CHAR)
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
ORDER BY s.Seadate DESC, s.Seaid DESC
LIMIT 1
UNION ALL
SELECT 'CONSOMMATION_ALCOOL', 'alcoolemia', CAST(c.Alcalcoolemie AS CHAR), CAST(c.Alcdateheure AS CHAR)
FROM CONSOMMATION_ALCOOL c
WHERE c.Usrid = ?
ORDER BY c.Alcdateheure DESC, c.Alcid DESC
LIMIT 1;

-- getActiveAlertsAndReminders
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
	AND dh.DHdate = CURDATE();

-- -----------------------------------------------------------
-- FINANCES
-- -----------------------------------------------------------

-- getUserAccounts
SELECT c.*
FROM COMPTE c
WHERE c.Usrid = ?
ORDER BY c.Comnom ASC;

-- getAccountBalances
SELECT c.Comid, c.Comnom, c.Comsolde
FROM COMPTE c
WHERE c.Usrid = ?
ORDER BY c.Comnom ASC;

-- getUserSubAccounts
-- TODO: aucun drapeau de sous-compte n'existe dans le schéma.
SELECT c.Comid, c.Comnom, c.Comsolde
FROM COMPTE c
WHERE c.Usrid = ?
ORDER BY c.Comnom ASC;

-- getFinanceTypes
SELECT t.*
FROM TYPE t
ORDER BY t.Typtitre ASC;

-- getTransactionsByPeriod
SELECT m.*
FROM MOUVEMENT m
WHERE m.Usrid = ?
	AND m.Moudate BETWEEN ? AND ?
ORDER BY m.Moudate DESC, m.Mouid DESC;

-- getTransactionsByAccount
SELECT m.*
FROM MOUVEMENT m
WHERE m.Usrid = ?
	AND m.Comid = ?
ORDER BY m.Moudate DESC, m.Mouid DESC;

-- getTransactionsByType
SELECT m.*
FROM MOUVEMENT m
WHERE m.Usrid = ?
	AND m.Typid = ?
ORDER BY m.Moudate DESC, m.Mouid DESC;

-- getIncomeTotalByPeriod
SELECT COALESCE(SUM(CASE WHEN m.Moumontant > 0 THEN m.Moumontant ELSE 0 END), 0) AS income_total
FROM MOUVEMENT m
WHERE m.Usrid = ?
	AND m.Moudate BETWEEN ? AND ?;

-- getExpenseTotalByPeriod
SELECT COALESCE(SUM(CASE WHEN m.Moumontant < 0 THEN ABS(m.Moumontant) ELSE 0 END), 0) AS expense_total
FROM MOUVEMENT m
WHERE m.Usrid = ?
	AND m.Moudate BETWEEN ? AND ?;

-- getNetBalanceByPeriod
SELECT COALESCE(SUM(m.Moumontant), 0) AS net_balance
FROM MOUVEMENT m
WHERE m.Usrid = ?
	AND m.Moudate BETWEEN ? AND ?;

-- getPlannedExpenses
SELECT f.*
FROM FACTURE f
WHERE f.Usrid = ?
ORDER BY f.FacdateProchain ASC;

-- getUpcomingPlannedExpenses
SELECT f.*
FROM FACTURE f
WHERE f.Usrid = ?
	AND f.FacdateProchain IS NOT NULL
	AND f.FacdateProchain >= CURDATE()
ORDER BY f.FacdateProchain ASC;

-- getPastPlannedExpenses
SELECT f.*
FROM FACTURE f
WHERE f.Usrid = ?
	AND f.FacdateProchain IS NOT NULL
	AND f.FacdateProchain < CURDATE()
ORDER BY f.FacdateProchain DESC;

-- getRecurringBillsByAccount
SELECT f.*
FROM FACTURE f
WHERE f.Usrid = ?
	AND f.Comid = ?
ORDER BY f.FacdateProchain ASC;

-- getFinancialGoals
-- TODO: nécessite une table d'objectifs absente du schéma.
SELECT NULL AS goal_id, NULL AS goal_name, NULL AS goal_value
WHERE 1 = 0;

-- getGoalProgressComparison
-- TODO: nécessite une table d'objectifs et des champs de suivi cibles.
SELECT NULL AS goal_name, NULL AS target_value, NULL AS actual_value
WHERE 1 = 0;

-- getTopExpenseTypes
SELECT t.Typid, t.Typtitre, COALESCE(SUM(ABS(m.Moumontant)), 0) AS total_expense
FROM MOUVEMENT m
JOIN TYPE t ON t.Typid = m.Typid
WHERE m.Usrid = ?
	AND m.Moudate BETWEEN ? AND ?
	AND m.Moumontant < 0
GROUP BY t.Typid, t.Typtitre
ORDER BY total_expense DESC
LIMIT ?;

-- getBalanceHistory
SELECT m.Moudate, SUM(m.Moumontant) OVER (ORDER BY m.Moudate, m.Mouid) AS running_balance
FROM MOUVEMENT m
WHERE m.Usrid = ?
	AND m.Moudate BETWEEN ? AND ?
ORDER BY m.Moudate ASC, m.Mouid ASC;

-- -----------------------------------------------------------
-- HABITUDES
-- -----------------------------------------------------------

-- getHabitCategories
SELECT c.*
FROM CATEGORIE c
ORDER BY c.Catnom ASC;

-- getActiveHabits
SELECT h.*
FROM HABITUDE h
WHERE h.Usrid = ?
ORDER BY h.Habnom ASC;

-- getPositiveHabits
SELECT h.*
FROM HABITUDE h
JOIN CATEGORIE c ON c.Catid = h.Catid
WHERE h.Usrid = ?
	AND c.Catplus = '1'
ORDER BY h.Habnom ASC;

-- getNegativeHabits
SELECT h.*
FROM HABITUDE h
JOIN CATEGORIE c ON c.Catid = h.Catid
WHERE h.Usrid = ?
	AND c.Catplus = '-1'
ORDER BY h.Habnom ASC;

-- getTodayHabits
SELECT h.Habid, h.Habnom, c.Catnom, c.Catplus, COALESCE(hb.HBdone, 0) AS done_today
FROM HABITUDE h
LEFT JOIN CATEGORIE c ON c.Catid = h.Catid
LEFT JOIN BILAN b ON b.Usrid = h.Usrid AND b.Bildate = CURDATE()
LEFT JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid AND hb.Habid = h.Habid
WHERE h.Usrid = ?
ORDER BY h.Habnom ASC;

-- getCompletedHabitsToday
SELECT h.Habid, h.Habnom, c.Catnom, c.Catplus
FROM HABITUDE h
JOIN CATEGORIE c ON c.Catid = h.Catid
JOIN BILAN b ON b.Usrid = h.Usrid AND b.Bildate = CURDATE()
JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid AND hb.Habid = h.Habid
WHERE h.Usrid = ?
	AND hb.HBdone = 1
ORDER BY h.Habnom ASC;

-- getPendingHabitsToday
SELECT h.Habid, h.Habnom, c.Catnom, c.Catplus
FROM HABITUDE h
JOIN CATEGORIE c ON c.Catid = h.Catid
LEFT JOIN BILAN b ON b.Usrid = h.Usrid AND b.Bildate = CURDATE()
LEFT JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid AND hb.Habid = h.Habid
WHERE h.Usrid = ?
	AND COALESCE(hb.HBdone, 0) = 0
ORDER BY h.Habnom ASC;

-- getTodayHabitSummary
SELECT
		COUNT(*) AS total_habits,
		COALESCE(SUM(CASE WHEN COALESCE(hb.HBdone, 0) = 1 THEN 1 ELSE 0 END), 0) AS completed_habits,
		COALESCE(SUM(CASE WHEN COALESCE(hb.HBdone, 0) = 0 THEN 1 ELSE 0 END), 0) AS pending_habits
FROM HABITUDE h
LEFT JOIN BILAN b ON b.Usrid = h.Usrid AND b.Bildate = CURDATE()
LEFT JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid AND hb.Habid = h.Habid
WHERE h.Usrid = ?;

-- getTodayHabitScore
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
WHERE h.Usrid = ?;

-- getWeeklyHabitScore
SELECT
		b.Bildate,
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
ORDER BY b.Bildate ASC;

-- getHabitHistory
SELECT b.Bildate, hb.HBdone
FROM BILAN b
JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid
WHERE b.Usrid = ?
	AND hb.Habid = ?
ORDER BY b.Bildate DESC;

-- getHabitCompletionRate
SELECT
		COALESCE(AVG(hb.HBdone) * 100, 0) AS completion_rate
FROM BILAN b
JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid
WHERE b.Usrid = ?
	AND hb.Habid = ?;

-- getHabitCurrentStreak
WITH RECURSIVE habit_days AS (
		SELECT CURDATE() AS day_value
		UNION ALL
		SELECT DATE_SUB(day_value, INTERVAL 1 DAY)
		FROM habit_days
		WHERE day_value > DATE_SUB(CURDATE(), INTERVAL 365 DAY)
),
done_days AS (
		SELECT hd.day_value
		FROM habit_days hd
		JOIN BILAN b ON b.Usrid = ? AND b.Bildate = hd.day_value
		JOIN HABITUDE_BILAN hb ON hb.Bilid = b.Bilid AND hb.Habid = ? AND hb.HBdone = 1
),
streak_anchor AS (
		SELECT day_value
		FROM done_days
		ORDER BY day_value DESC
		LIMIT 1
)
SELECT COUNT(*) AS current_streak
FROM done_days
WHERE day_value >= (SELECT COALESCE(MIN(day_value), CURDATE()) FROM streak_anchor);

-- getHabitsByFrequency
-- TODO: aucun champ de fréquence n'existe dans le schéma.
SELECT h.Habid, h.Habnom, COUNT(hb.Bilid) AS checked_days
FROM HABITUDE h
LEFT JOIN HABITUDE_BILAN hb ON hb.Habid = h.Habid
WHERE h.Usrid = ?
GROUP BY h.Habid, h.Habnom
ORDER BY checked_days DESC;

-- getWeeklyHabits
SELECT h.*
FROM HABITUDE h
WHERE h.Usrid = ?
ORDER BY h.Habnom ASC;

-- getCustomHabits
SELECT h.*
FROM HABITUDE h
WHERE h.Usrid = ?
ORDER BY h.Habnom ASC;

-- getArchivedHabits
-- TODO: aucun drapeau d'archivage n'existe dans le schéma.
SELECT NULL AS habit_id, NULL AS habit_name
WHERE 1 = 0;

-- getMostConsistentHabits
SELECT h.Habid, h.Habnom, COALESCE(AVG(hb.HBdone) * 100, 0) AS consistency_rate
FROM HABITUDE h
LEFT JOIN HABITUDE_BILAN hb ON hb.Habid = h.Habid
WHERE h.Usrid = ?
GROUP BY h.Habid, h.Habnom
ORDER BY consistency_rate DESC
LIMIT ?;

-- getLeastConsistentHabits
SELECT h.Habid, h.Habnom, COALESCE(AVG(hb.HBdone) * 100, 0) AS consistency_rate
FROM HABITUDE h
LEFT JOIN HABITUDE_BILAN hb ON hb.Habid = h.Habid
WHERE h.Usrid = ?
GROUP BY h.Habid, h.Habnom
ORDER BY consistency_rate ASC
LIMIT ?;

-- -----------------------------------------------------------
-- SOBRIÉTÉ
-- -----------------------------------------------------------

-- getCurrentSobrietyPeriod
SELECT s.*
FROM SOBRIETE s
WHERE s.Usrid = ?
	AND s.Sobfin IS NULL
ORDER BY s.Sobdebut DESC
LIMIT 1;

-- getCurrentSobrietyDuration
SELECT TIMESTAMPDIFF(HOUR, s.Sobdebut, NOW()) AS sobriety_hours
FROM SOBRIETE s
WHERE s.Usrid = ?
	AND s.Sobfin IS NULL
ORDER BY s.Sobdebut DESC
LIMIT 1;

-- getSobrietyHistory
SELECT s.*
FROM SOBRIETE s
WHERE s.Usrid = ?
ORDER BY s.Sobdebut DESC;

-- getSobrietyTemplates
-- TODO: nécessite une table de modèles.
SELECT NULL AS template_id, NULL AS template_name
WHERE 1 = 0;

-- getSelectedSobrietyTemplate
-- TODO: nécessite un champ/table de modèle sélectionné.
SELECT NULL AS template_id, NULL AS template_name
WHERE 1 = 0;

-- getCurrentSobrietyStartDate
SELECT s.Sobdebut
FROM SOBRIETE s
WHERE s.Usrid = ?
	AND s.Sobfin IS NULL
ORDER BY s.Sobdebut DESC
LIMIT 1;

-- getLastSobrietyBreak
SELECT s.Sobfin
FROM SOBRIETE s
WHERE s.Usrid = ?
	AND s.Sobfin IS NOT NULL
ORDER BY s.Sobfin DESC
LIMIT 1;

-- getTotalSobrietyDuration
SELECT COALESCE(SUM(TIMESTAMPDIFF(HOUR, s.Sobdebut, COALESCE(s.Sobfin, NOW()))), 0) AS total_sobriety_hours
FROM SOBRIETE s
WHERE s.Usrid = ?;

-- getSobrietyStatsByPeriod
SELECT
		COUNT(*) AS sobriety_period_count,
		COALESCE(SUM(TIMESTAMPDIFF(HOUR, s.Sobdebut, COALESCE(s.Sobfin, NOW()))), 0) AS sobriety_hours
FROM SOBRIETE s
WHERE s.Usrid = ?
	AND s.Sobdebut >= DATE_SUB(NOW(), INTERVAL ? DAY);

-- -----------------------------------------------------------
-- HUMEUR
-- -----------------------------------------------------------

-- getMoodTypes
SELECT h.*
FROM HUMEUR h
ORDER BY h.Humnom ASC;

-- getMoodColors
SELECT h.Humid, h.Humnom, h.Humcolor
FROM HUMEUR h
ORDER BY h.Humnom ASC;

-- getTodayMood
SELECT dh.*
FROM DATE_HUMEUR dh
WHERE dh.Usrid = ?
	AND dh.DHdate = CURDATE();

-- getMoodByDate
SELECT dh.*
FROM DATE_HUMEUR dh
WHERE dh.Usrid = ?
	AND dh.DHdate = ?;

-- getMonthlyMoods
SELECT dh.*
FROM DATE_HUMEUR dh
WHERE dh.Usrid = ?
	AND dh.DHdate BETWEEN ? AND ?
ORDER BY dh.DHdate ASC;

-- getMoodsByPeriod
SELECT dh.*
FROM DATE_HUMEUR dh
WHERE dh.Usrid = ?
	AND dh.DHdate BETWEEN ? AND ?
ORDER BY dh.DHdate ASC;

-- getMissingMoodDays
WITH RECURSIVE mood_days AS (
		SELECT ? AS day_value
		UNION ALL
		SELECT DATE_ADD(day_value, INTERVAL 1 DAY)
		FROM mood_days
		WHERE day_value < ?
)
SELECT md.day_value
FROM mood_days md
LEFT JOIN DATE_HUMEUR dh ON dh.Usrid = ? AND dh.DHdate = md.day_value
WHERE dh.DHdate IS NULL;

-- getMostFrequentMood
SELECT dh.Humid, h.Humnom, COUNT(*) AS mood_count
FROM DATE_HUMEUR dh
JOIN HUMEUR h ON h.Humid = dh.Humid
WHERE dh.Usrid = ?
GROUP BY dh.Humid, h.Humnom
ORDER BY mood_count DESC
LIMIT 1;

-- getMoodDistributionByPeriod
SELECT h.Humid, h.Humnom, COUNT(*) AS mood_count
FROM DATE_HUMEUR dh
JOIN HUMEUR h ON h.Humid = dh.Humid
WHERE dh.Usrid = ?
	AND dh.DHdate BETWEEN ? AND ?
GROUP BY h.Humid, h.Humnom
ORDER BY mood_count DESC;

-- getCustomMoods
-- TODO: le schéma ne distingue pas les humeurs personnalisées des humeurs par défaut.
SELECT NULL AS mood_id, NULL AS mood_name, NULL AS mood_color
WHERE 1 = 0;

-- -----------------------------------------------------------
-- HYDRATATION
-- -----------------------------------------------------------

-- getTodayHydration
SELECT h.*
FROM HYDRATATION h
WHERE h.Usrid = ?
	AND h.Hyddate = CURDATE();

-- getTodayHydrationGoal
SELECT h.Hydobjectif
FROM HYDRATATION h
WHERE h.Usrid = ?
	AND h.Hyddate = CURDATE()
ORDER BY h.Hydid DESC
LIMIT 1;

-- getDefaultHydrationGoal
-- TODO: aucune table d'objectif d'hydratation par défaut pour l'utilisateur n'existe.
SELECT NULL AS default_goal
WHERE 1 = 0;

-- getHydrationHistory
SELECT h.*
FROM HYDRATATION h
WHERE h.Usrid = ?
ORDER BY h.Hyddate DESC;

-- getTodayWaterTotal
SELECT COALESCE(SUM(h.Hydquantite), 0) AS water_total
FROM HYDRATATION h
WHERE h.Usrid = ?
	AND h.Hyddate = CURDATE();

-- getHydrationGoalProgress
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
) AS goal;

-- getHydrationGoalReachedDays
SELECT h.Hyddate
FROM HYDRATATION h
WHERE h.Usrid = ?
	AND h.Hydquantite >= h.Hydobjectif
ORDER BY h.Hyddate DESC;

-- getHydrationGoalMissedDays
SELECT h.Hyddate
FROM HYDRATATION h
WHERE h.Usrid = ?
	AND h.Hydquantite < h.Hydobjectif
ORDER BY h.Hyddate DESC;

-- getWeeklyHydrationAverage
SELECT ROUND(AVG(day_total), 2) AS weekly_average
FROM (
		SELECT h.Hyddate, SUM(h.Hydquantite) AS day_total
		FROM HYDRATATION h
		WHERE h.Usrid = ?
			AND h.Hyddate BETWEEN DATE_SUB(CURDATE(), INTERVAL 6 DAY) AND CURDATE()
		GROUP BY h.Hyddate
) AS daily;

-- getMonthlyHydrationAverage
SELECT ROUND(AVG(day_total), 2) AS monthly_average
FROM (
		SELECT h.Hyddate, SUM(h.Hydquantite) AS day_total
		FROM HYDRATATION h
		WHERE h.Usrid = ?
			AND h.Hyddate BETWEEN DATE_SUB(CURDATE(), INTERVAL 29 DAY) AND CURDATE()
		GROUP BY h.Hyddate
) AS daily;

-- getMostUsedWaterInputs
-- TODO: no quick-input table exists in the schema.
SELECT NULL AS input_volume, NULL AS usage_count
WHERE 1 = 0;

-- getHydrationGoalHistory
SELECT h.Hyddate, h.Hydobjectif
FROM HYDRATATION h
WHERE h.Usrid = ?
ORDER BY h.Hyddate ASC;

-- -----------------------------------------------------------
-- SOMMEIL
-- -----------------------------------------------------------

-- getLatestSleepEntry
SELECT s.*
FROM SOMMEIL s
WHERE s.Usrid = ?
ORDER BY s.Somdate DESC, s.Somid DESC
LIMIT 1;

-- getTodaySleep
SELECT s.*
FROM SOMMEIL s
WHERE s.Usrid = ?
	AND s.Somdate = CURDATE()
ORDER BY s.Somid DESC
LIMIT 1;

-- getSleepHistory
SELECT s.*
FROM SOMMEIL s
WHERE s.Usrid = ?
ORDER BY s.Somdate DESC, s.Somid DESC;

-- getSleepDuration
SELECT COALESCE(s.Somduree, TIMESTAMPDIFF(MINUTE, s.Somcoucher, s.Somlever)) AS sleep_duration_minutes
FROM SOMMEIL s
WHERE s.Usrid = ?
	AND s.Somid = ?;

-- getRestfulSleepEntries
SELECT s.*
FROM SOMMEIL s
WHERE s.Usrid = ?
	AND s.Somreposant = 1
ORDER BY s.Somdate DESC;

-- getNonRestfulSleepEntries
SELECT s.*
FROM SOMMEIL s
WHERE s.Usrid = ?
	AND s.Somreposant = 0
ORDER BY s.Somdate DESC;

-- getAverageBedtime
SELECT SEC_TO_TIME(AVG(TIME_TO_SEC(s.Somcoucher))) AS average_bedtime
FROM SOMMEIL s
WHERE s.Usrid = ?
	AND s.Somdate BETWEEN DATE_SUB(CURDATE(), INTERVAL 6 DAY) AND CURDATE();

-- getAverageWakeTime
SELECT SEC_TO_TIME(AVG(TIME_TO_SEC(s.Somlever))) AS average_wake_time
FROM SOMMEIL s
WHERE s.Usrid = ?
	AND s.Somdate BETWEEN DATE_SUB(CURDATE(), INTERVAL 6 DAY) AND CURDATE();

-- getWeeklySleepAverage
SELECT ROUND(AVG(s.Somduree), 2) AS weekly_sleep_average
FROM SOMMEIL s
WHERE s.Usrid = ?
	AND s.Somdate BETWEEN DATE_SUB(CURDATE(), INTERVAL 6 DAY) AND CURDATE();

-- getMonthlySleepAverage
SELECT ROUND(AVG(s.Somduree), 2) AS monthly_sleep_average
FROM SOMMEIL s
WHERE s.Usrid = ?
	AND s.Somdate BETWEEN DATE_SUB(CURDATE(), INTERVAL 29 DAY) AND CURDATE();

-- getSleepReminderSettings
-- TODO: nécessite les paramètres de rappel utilisateur.
SELECT NULL AS reminder_key, NULL AS reminder_value
WHERE 1 = 0;

-- getShortSleepEntries
SELECT s.*
FROM SOMMEIL s
WHERE s.Usrid = ?
	AND COALESCE(s.Somduree, TIMESTAMPDIFF(MINUTE, s.Somcoucher, s.Somlever)) < ?
ORDER BY s.Somdate DESC;

-- getIrregularSleepPatterns
SELECT s.*
FROM SOMMEIL s
WHERE s.Usrid = ?
	AND s.Somdate BETWEEN ? AND ?
ORDER BY s.Somdate ASC;

-- -----------------------------------------------------------
-- REPAS / NUTRITION
-- -----------------------------------------------------------

-- getTodayMeals
SELECT r.*
FROM REPAS r
WHERE r.Usrid = ?
	AND r.Repdate = CURDATE()
ORDER BY r.Repid DESC;

-- getMealsByPeriod
SELECT r.*
FROM REPAS r
WHERE r.Usrid = ?
	AND r.Repdate BETWEEN ? AND ?
ORDER BY r.Repdate ASC, r.Repid ASC;

-- getLatestMeal
SELECT r.*
FROM REPAS r
WHERE r.Usrid = ?
ORDER BY r.Repdate DESC, r.Repid DESC
LIMIT 1;

-- getTodayCalorieTotal
SELECT COALESCE(SUM(r.Repcalories), 0) AS calorie_total
FROM REPAS r
WHERE r.Usrid = ?
	AND r.Repdate = CURDATE();

-- getTodayProteinTotal
SELECT COALESCE(SUM(r.Repproteines), 0) AS protein_total
FROM REPAS r
WHERE r.Usrid = ?
	AND r.Repdate = CURDATE();

-- getTodayCarbTotal
SELECT COALESCE(SUM(r.Repglucides), 0) AS carb_total
FROM REPAS r
WHERE r.Usrid = ?
	AND r.Repdate = CURDATE();

-- getTodayFatTotal
SELECT COALESCE(SUM(r.Replipides), 0) AS fat_total
FROM REPAS r
WHERE r.Usrid = ?
	AND r.Repdate = CURDATE();

-- getTodayCalorieGoal
-- TODO: aucune table d'objectif calorique n'existe dans le schéma.
SELECT NULL AS calorie_goal
WHERE 1 = 0;

-- getRemainingCalories
-- TODO: nécessite une source d'objectif calorique quotidien.
SELECT NULL AS remaining_calories
WHERE 1 = 0;

-- getVisibleMacroSettings
-- TODO: nécessite des paramètres de personnalisation.
SELECT NULL AS macro_key, NULL AS macro_visible
WHERE 1 = 0;

-- getDailyMacroDistribution
SELECT
		COALESCE(SUM(r.Repproteines), 0) AS proteins,
		COALESCE(SUM(r.Repglucides), 0) AS carbs,
		COALESCE(SUM(r.Replipides), 0) AS fats
FROM REPAS r
WHERE r.Usrid = ?
	AND r.Repdate = CURDATE();

-- getNutritionHistory
SELECT r.Repdate,
			 COALESCE(SUM(r.Repcalories), 0) AS calories,
			 COALESCE(SUM(r.Repproteines), 0) AS proteins,
			 COALESCE(SUM(r.Repglucides), 0) AS carbs,
			 COALESCE(SUM(r.Replipides), 0) AS fats
FROM REPAS r
WHERE r.Usrid = ?
	AND r.Repdate BETWEEN ? AND ?
GROUP BY r.Repdate
ORDER BY r.Repdate ASC;

-- getWeeklyCalorieAverage
SELECT ROUND(AVG(day_calories), 2) AS weekly_calorie_average
FROM (
		SELECT r.Repdate, COALESCE(SUM(r.Repcalories), 0) AS day_calories
		FROM REPAS r
		WHERE r.Usrid = ?
			AND r.Repdate BETWEEN DATE_SUB(CURDATE(), INTERVAL 6 DAY) AND CURDATE()
		GROUP BY r.Repdate
) AS daily;

-- getMonthlyCalorieAverage
SELECT ROUND(AVG(day_calories), 2) AS monthly_calorie_average
FROM (
		SELECT r.Repdate, COALESCE(SUM(r.Repcalories), 0) AS day_calories
		FROM REPAS r
		WHERE r.Usrid = ?
			AND r.Repdate BETWEEN DATE_SUB(CURDATE(), INTERVAL 29 DAY) AND CURDATE()
		GROUP BY r.Repdate
) AS daily;

-- -----------------------------------------------------------
-- MESURES CORPORELLES
-- -----------------------------------------------------------

-- getLatestBodyMeasurement
SELECT m.*
FROM MESURE_CORPORELLE m
WHERE m.Usrid = ?
ORDER BY m.Mesdate DESC, m.Mesid DESC
LIMIT 1;

-- getWeightHistory
SELECT m.Mesdate, m.Mespoids
FROM MESURE_CORPORELLE m
WHERE m.Usrid = ?
ORDER BY m.Mesdate ASC, m.Mesid ASC;

-- getHeightHistory
SELECT m.Mesdate, m.Mestaille
FROM MESURE_CORPORELLE m
WHERE m.Usrid = ?
	AND m.Mestaille IS NOT NULL
ORDER BY m.Mesdate ASC, m.Mesid ASC;

-- getWeightChartData
SELECT m.Mesdate, m.Mespoids
FROM MESURE_CORPORELLE m
WHERE m.Usrid = ?
	AND m.Mesdate BETWEEN ? AND ?
ORDER BY m.Mesdate ASC;

-- getHeightChartData
SELECT m.Mesdate, m.Mestaille
FROM MESURE_CORPORELLE m
WHERE m.Usrid = ?
	AND m.Mestaille IS NOT NULL
	AND m.Mesdate BETWEEN ? AND ?
ORDER BY m.Mesdate ASC;

-- getWeightProgress
SELECT
		first_weight,
		last_weight,
		last_weight - first_weight AS weight_delta
FROM (
		SELECT
				(SELECT m.Mespoids FROM MESURE_CORPORELLE m WHERE m.Usrid = ? ORDER BY m.Mesdate ASC, m.Mesid ASC LIMIT 1) AS first_weight,
				(SELECT m.Mespoids FROM MESURE_CORPORELLE m WHERE m.Usrid = ? ORDER BY m.Mesdate DESC, m.Mesid DESC LIMIT 1) AS last_weight
) AS weights;

-- getWeightVariationBetweenDates
SELECT
		start_row.Mespoids AS start_weight,
		end_row.Mespoids AS end_weight,
		end_row.Mespoids - start_row.Mespoids AS weight_variation
FROM (
		SELECT m.Mespoids
		FROM MESURE_CORPORELLE m
		WHERE m.Usrid = ? AND m.Mesdate <= ?
		ORDER BY m.Mesdate DESC, m.Mesid DESC
		LIMIT 1
) AS start_row
CROSS JOIN (
		SELECT m.Mespoids
		FROM MESURE_CORPORELLE m
		WHERE m.Usrid = ? AND m.Mesdate <= ?
		ORDER BY m.Mesdate DESC, m.Mesid DESC
		LIMIT 1
) AS end_row;

-- getCurrentBmi
SELECT m.MesIMC
FROM MESURE_CORPORELLE m
WHERE m.Usrid = ?
ORDER BY m.Mesdate DESC, m.Mesid DESC
LIMIT 1;

-- getBmiHistory
SELECT m.Mesdate, m.MesIMC
FROM MESURE_CORPORELLE m
WHERE m.Usrid = ?
	AND m.MesIMC IS NOT NULL
ORDER BY m.Mesdate ASC;

-- getCurrentBasalMetabolism
SELECT m.MesMetaBasal
FROM MESURE_CORPORELLE m
WHERE m.Usrid = ?
ORDER BY m.Mesdate DESC, m.Mesid DESC
LIMIT 1;

-- getBasalMetabolismHistory
SELECT m.Mesdate, m.MesMetaBasal
FROM MESURE_CORPORELLE m
WHERE m.Usrid = ?
	AND m.MesMetaBasal IS NOT NULL
ORDER BY m.Mesdate ASC;

-- getTargetCalorieDeficit
-- TODO: nécessite une table d'objectif ou cible pour comparaison.
SELECT NULL AS target_calorie_deficit
WHERE 1 = 0;

-- getPonderalCalculation
-- TODO: aucune table de cible pondérale dédiée n'existe.
SELECT NULL AS ponderal_value
WHERE 1 = 0;

-- getEstimatedNeat
-- TODO: nécessite une table de règles d'estimation NEAT ou des entrées externes.
SELECT NULL AS neat_value
WHERE 1 = 0;

-- getHealthDerivedMetrics
SELECT
		latest.Mespoids AS latest_weight,
		latest.Mestaille AS latest_height,
		latest.MesIMC AS latest_bmi,
		latest.MesMetaBasal AS latest_basal_metabolism
FROM MESURE_CORPORELLE latest
WHERE latest.Usrid = ?
ORDER BY latest.Mesdate DESC, latest.Mesid DESC
LIMIT 1;

-- -----------------------------------------------------------
-- SPORT
-- -----------------------------------------------------------

-- getSportTypes
SELECT s.*
FROM SPORT_TYPE s
ORDER BY s.Stypnom ASC;

-- getCustomSports
-- TODO: aucun drapeau de sport personnalisé n'existe dans le schéma.
SELECT s.*
FROM SPORT_TYPE s
ORDER BY s.Stypnom ASC;

-- getTodaySportSessions
SELECT s.*
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
	AND s.Seadate = CURDATE()
ORDER BY s.Seaid DESC;

-- getSportSessionsByPeriod
SELECT s.*
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
	AND s.Seadate BETWEEN ? AND ?
ORDER BY s.Seadate ASC, s.Seaid ASC;

-- getLatestSportSession
SELECT s.*
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
ORDER BY s.Seadate DESC, s.Seaid DESC
LIMIT 1;

-- getTotalSportDurationByPeriod
SELECT COALESCE(SUM(s.Seaduree), 0) AS sport_duration_total
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
	AND s.Seadate BETWEEN ? AND ?;

-- getSportSessionCountByPeriod
SELECT COUNT(*) AS sport_session_count
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
	AND s.Seadate BETWEEN ? AND ?;

-- getBurnedCaloriesByPeriod
SELECT COALESCE(SUM(s.Seacalories), 0) AS burned_calories
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
	AND s.Seadate BETWEEN ? AND ?;

-- getSportSessionsByIntensity
SELECT s.*
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
	AND s.Seaintensite = ?
ORDER BY s.Seadate DESC, s.Seaid DESC;

-- getSportSessionsByType
SELECT s.*
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
	AND s.Stypid = ?
ORDER BY s.Seadate DESC, s.Seaid DESC;

-- getMostPracticedSport
SELECT st.Stypid, st.Stypnom, COUNT(*) AS session_count
FROM SEANCE_SPORT ss
LEFT JOIN SPORT_TYPE st ON st.Stypid = ss.Stypid
WHERE ss.Usrid = ?
GROUP BY st.Stypid, st.Stypnom
ORDER BY session_count DESC
LIMIT 1;

-- getWeeklySportStats
SELECT
		COUNT(*) AS session_count,
		COALESCE(SUM(s.Seaduree), 0) AS total_duration,
		COALESCE(SUM(s.Seacalories), 0) AS total_calories
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
	AND s.Seadate BETWEEN DATE_SUB(CURDATE(), INTERVAL 6 DAY) AND CURDATE();

-- getMonthlySportStats
SELECT
		COUNT(*) AS session_count,
		COALESCE(SUM(s.Seaduree), 0) AS total_duration,
		COALESCE(SUM(s.Seacalories), 0) AS total_calories
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
	AND s.Seadate BETWEEN DATE_SUB(CURDATE(), INTERVAL 29 DAY) AND CURDATE();

-- getSportChartData
SELECT s.Seadate, COALESCE(SUM(s.Seaduree), 0) AS duration, COALESCE(SUM(s.Seacalories), 0) AS calories
FROM SEANCE_SPORT s
WHERE s.Usrid = ?
	AND s.Seadate BETWEEN ? AND ?
GROUP BY s.Seadate
ORDER BY s.Seadate ASC;

-- -----------------------------------------------------------
-- RESPIRATION / COHÉRENCE CARDIAQUE
-- -----------------------------------------------------------

-- getLatestBreathingSession
SELECT c.*
FROM COHERENCE_CARDIAQUE c
WHERE c.Usrid = ?
ORDER BY c.Cohdateheure DESC, c.Cohid DESC
LIMIT 1;

-- getTodayBreathingSessions
SELECT c.*
FROM COHERENCE_CARDIAQUE c
WHERE c.Usrid = ?
	AND DATE(c.Cohdateheure) = CURDATE()
ORDER BY c.Cohdateheure DESC, c.Cohid DESC;

-- getBreathingSessionsByPeriod
SELECT c.*
FROM COHERENCE_CARDIAQUE c
WHERE c.Usrid = ?
	AND DATE(c.Cohdateheure) BETWEEN ? AND ?
ORDER BY c.Cohdateheure ASC;

-- getTotalBreathingDurationByPeriod
SELECT COALESCE(SUM(c.Cohduree), 0) AS breathing_duration_total
FROM COHERENCE_CARDIAQUE c
WHERE c.Usrid = ?
	AND DATE(c.Cohdateheure) BETWEEN ? AND ?;

-- getBreathingSessionCountByPeriod
SELECT COUNT(*) AS breathing_session_count
FROM COHERENCE_CARDIAQUE c
WHERE c.Usrid = ?
	AND DATE(c.Cohdateheure) BETWEEN ? AND ?;

-- getBreathingCircleSettings
-- TODO: nécessite une table de paramètres pour la visualisation du cercle.
SELECT NULL AS setting_key, NULL AS setting_value
WHERE 1 = 0;

-- getActiveBreathingCircleCustomization
-- TODO: nécessite des tables de personnalisation sauvegardées.
SELECT NULL AS customization_key, NULL AS customization_value
WHERE 1 = 0;

-- getSavedBreathingConfigurations
-- TODO: nécessite des tables de configuration sauvegardées.
SELECT NULL AS configuration_id, NULL AS configuration_name
WHERE 1 = 0;

-- getAverageBreathingUsageFrequency
SELECT ROUND(COUNT(*) / NULLIF(COUNT(DISTINCT DATE(c.Cohdateheure)), 0), 2) AS average_frequency
FROM COHERENCE_CARDIAQUE c
WHERE c.Usrid = ?;

-- getStressLinkedBreathingSessions
-- TODO: nécessite des événements de stress ou des étiquettes pour corréler les sessions.
SELECT NULL AS session_id, NULL AS stress_link
WHERE 1 = 0;

-- -----------------------------------------------------------
-- ALCOOL
-- -----------------------------------------------------------

-- getLatestAlcoholEntry
SELECT c.*
FROM CONSOMMATION_ALCOOL c
WHERE c.Usrid = ?
ORDER BY c.Alcdateheure DESC, c.Alcid DESC
LIMIT 1;

-- getAlcoholEntriesByPeriod
SELECT c.*
FROM CONSOMMATION_ALCOOL c
WHERE c.Usrid = ?
	AND c.Alcdateheure BETWEEN ? AND ?
ORDER BY c.Alcdateheure ASC, c.Alcid ASC;

-- getCurrentBloodAlcoholLevel
SELECT c.Alcalcoolemie
FROM CONSOMMATION_ALCOOL c
WHERE c.Usrid = ?
ORDER BY c.Alcdateheure DESC, c.Alcid DESC
LIMIT 1;

-- getTimeUntilSobriety
SELECT c.Alctempsobre
FROM CONSOMMATION_ALCOOL c
WHERE c.Usrid = ?
ORDER BY c.Alcdateheure DESC, c.Alcid DESC
LIMIT 1;
