-- =====================================================================
--                   LIFETRACK - REQUÊTES INSERT
--                  Ajout de données à la base
-- =====================================================================

-- =====================================================================
--                         GLOBAL DOMAIN
-- =====================================================================

-- Créer un utilisateur
INSERT INTO UTILISATEUR (UsrpublicId, UsrcreatedAt)
VALUES ('user_001', NOW());

-- =====================================================================
--                       FINANCES DOMAIN
-- =====================================================================

-- Créer un compte bancaire
INSERT INTO COMPTE (Usrid, Comnom, Comsolde)
VALUES (1, 'Compte Courant', 1500.00);

INSERT INTO COMPTE (Usrid, Comnom, Comsolde)
VALUES (1, 'Compte Épargne', 5000.00);

-- Créer les types de transactions
INSERT INTO TYPE (Typtitre)
VALUES ('Salaire');

INSERT INTO TYPE (Typtitre)
VALUES ('Alimentation');

INSERT INTO TYPE (Typtitre)
VALUES ('Transport');

INSERT INTO TYPE (Typtitre)
VALUES ('Loisir');

INSERT INTO TYPE (Typtitre)
VALUES ('Santé');

INSERT INTO TYPE (Typtitre)
VALUES ('Logement');

INSERT INTO TYPE (Typtitre)
VALUES ('Utilitaires');

-- Ajouter des transactions (mouvements)
-- Transaction positive (revenu)
INSERT INTO MOUVEMENT (Usrid, Comid, Typid, Moudate, Moumontant, Moudescription)
VALUES (1, 1, 1, '2026-05-01', 2500.00, 'Salaire mensuel');

-- Transactions négatives (dépenses)
INSERT INTO MOUVEMENT (Usrid, Comid, Typid, Moudate, Moumontant, Moudescription)
VALUES (1, 1, 2, '2026-05-02', -45.50, 'Courses alimentaires');

INSERT INTO MOUVEMENT (Usrid, Comid, Typid, Moudate, Moumontant, Moudescription)
VALUES (1, 1, 3, '2026-05-02', -25.00, 'Transport en commun');

INSERT INTO MOUVEMENT (Usrid, Comid, Typid, Moudate, Moumontant, Moudescription)
VALUES (1, 1, 4, '2026-05-03', -60.00, 'Cinema et loisirs');

INSERT INTO MOUVEMENT (Usrid, Comid, Typid, Moudate, Moumontant, Moudescription)
VALUES (1, 1, 5, '2026-05-05', -80.00, 'Consultation médicale');

INSERT INTO MOUVEMENT (Usrid, Comid, Typid, Moudate, Moumontant, Moudescription)
VALUES (1, 1, 6, '2026-05-01', -1000.00, 'Loyer mensuel');

INSERT INTO MOUVEMENT (Usrid, Comid, Typid, Moudate, Moumontant, Moudescription)
VALUES (1, 1, 7, '2026-05-04', -120.00, 'Facture électricité');

-- Ajouter une facture (dépense prévue)
INSERT INTO FACTURE (Usrid, Facles, FacdateProchain, Facmontant)
VALUES (1, 'Internet', '2026-06-01', 49.99);

INSERT INTO FACTURE (Usrid, Facles, FacdateProchain, Facmontant)
VALUES (1, 'Assurance Auto', '2026-06-15', 150.00);

-- =====================================================================
--                        HABITS DOMAIN
-- =====================================================================

-- Créer les catégories d'habitudes
INSERT INTO CATEGORIE_HABITUDE (Catnom)
VALUES ('Santé');

INSERT INTO CATEGORIE_HABITUDE (Catnom)
VALUES ('Productivité');

INSERT INTO CATEGORIE_HABITUDE (Catnom)
VALUES ('Bien-être');

INSERT INTO CATEGORIE_HABITUDE (Catnom)
VALUES ('Apprentissage');

-- Créer des habitudes positives
INSERT INTO HABITUDE (Usrid, Catid, Habtitre, Habdescription, Habtype)
VALUES (1, 1, 'Faire du sport', 'Faire 30 min d''activité physique', 'POSITIVE');

INSERT INTO HABITUDE (Usrid, Catid, Habtitre, Habdescription, Habtype)
VALUES (1, 1, 'Méditer', 'Session de méditation quotidienne', 'POSITIVE');

INSERT INTO HABITUDE (Usrid, Catid, Habtitre, Habdescription, Habtype)
VALUES (1, 2, 'Travailler sur le projet', 'Consacrer 2h au projet LifeTrack', 'POSITIVE');

INSERT INTO HABITUDE (Usrid, Catid, Habtitre, Habdescription, Habtype)
VALUES (1, 3, 'Lire', 'Lire 20 pages d''un livre', 'POSITIVE');

-- Créer des habitudes négatives à éviter
INSERT INTO HABITUDE (Usrid, Catid, Habtitre, Habdescription, Habtype)
VALUES (1, 3, 'Scrolling sur les réseaux', 'Limiter le temps sur les réseaux', 'NEGATIVE');

INSERT INTO HABITUDE (Usrid, Catid, Habtitre, Habdescription, Habtype)
VALUES (1, 1, 'Manger sucré', 'Limiter les aliments sucrés', 'NEGATIVE');

-- Marquer des habitudes comme complétées aujourd'hui
INSERT INTO JOURNEE_HABITUDE (Usrid, Habid, Jhadate, Jhadone)
VALUES (1, 1, CURDATE(), 1);

INSERT INTO JOURNEE_HABITUDE (Usrid, Habid, Jhadate, Jhadone)
VALUES (1, 2, CURDATE(), 1);

INSERT INTO JOURNEE_HABITUDE (Usrid, Habid, Jhadate, Jhadone)
VALUES (1, 3, CURDATE(), 0);

INSERT INTO JOURNEE_HABITUDE (Usrid, Habid, Jhadate, Jhadone)
VALUES (1, 4, CURDATE(), 1);

INSERT INTO JOURNEE_HABITUDE (Usrid, Habid, Jhadate, Jhadone)
VALUES (1, 5, CURDATE(), 0);

INSERT INTO JOURNEE_HABITUDE (Usrid, Habid, Jhadate, Jhadone)
VALUES (1, 6, CURDATE(), 1);

-- =====================================================================
--                       SOBRIETY DOMAIN
-- =====================================================================

-- Créer une période de sobriété
INSERT INTO PERIODE_SOBRIETE (Usrid, Perdatedebut, Perdatefin, Perstatus)
VALUES (1, '2026-01-01', NULL, 'EN_COURS');

-- =====================================================================
--                         MOOD DOMAIN
-- =====================================================================

-- Créer les types d'humeur
INSERT INTO TYPE_HUMEUR (Typnom)
VALUES ('Très heureux');

INSERT INTO TYPE_HUMEUR (Typnom)
VALUES ('Heureux');

INSERT INTO TYPE_HUMEUR (Typnom)
VALUES ('Neutre');

INSERT INTO TYPE_HUMEUR (Typnom)
VALUES ('Triste');

INSERT INTO TYPE_HUMEUR (Typnom)
VALUES ('Très triste');

-- Enregistrer l'humeur du jour
INSERT INTO DATE_HUMEUR (Usrid, DHdate, Typid, DHnotes)
VALUES (1, CURDATE(), 2, 'Bonne journée productive');

INSERT INTO DATE_HUMEUR (Usrid, DHdate, Typid, DHnotes)
VALUES (1, '2026-05-09', 2, 'Journée normale');

INSERT INTO DATE_HUMEUR (Usrid, DHdate, Typid, DHnotes)
VALUES (1, '2026-05-08', 3, 'Un peu fatigué');

-- =====================================================================
--                      HYDRATION DOMAIN
-- =====================================================================

-- Créer les objectifs d'hydratation
INSERT INTO HYDRATATION (Usrid, Hyddate, Hydquantite, Hydtype, Hydobjectif)
VALUES (1, CURDATE(), 250.00, 'Verre', 2500.00);

INSERT INTO HYDRATATION (Usrid, Hyddate, Hydquantite, Hydtype, Hydobjectif)
VALUES (1, CURDATE(), 250.00, 'Verre', 2500.00);

INSERT INTO HYDRATATION (Usrid, Hyddate, Hydquantite, Hydtype, Hydobjectif)
VALUES (1, CURDATE(), 500.00, 'Bouteille', 2500.00);

INSERT INTO HYDRATATION (Usrid, Hyddate, Hydquantite, Hydtype, Hydobjectif)
VALUES (1, CURDATE(), 250.00, 'Verre', 2500.00);

INSERT INTO HYDRATATION (Usrid, Hyddate, Hydquantite, Hydtype, Hydobjectif)
VALUES (1, '2026-05-09', 2000.00, 'Bouteille', 2500.00);

INSERT INTO HYDRATATION (Usrid, Hyddate, Hydquantite, Hydtype, Hydobjectif)
VALUES (1, '2026-05-08', 2200.00, 'Bouteille', 2500.00);

-- =====================================================================
--                         SLEEP DOMAIN
-- =====================================================================

-- Enregistrer des entrées de sommeil
INSERT INTO SOMMEIL (Usrid, Somdate, Somheure, Somduree, Somqualite, Somrestful)
VALUES (1, CURDATE(), '23:00:00', 480, 8, 1);

INSERT INTO SOMMEIL (Usrid, Somdate, Somheure, Somduree, Somqualite, Somrestful)
VALUES (1, '2026-05-09', '23:30:00', 450, 7.5, 1);

INSERT INTO SOMMEIL (Usrid, Somdate, Somheure, Somduree, Somqualite, Somrestful)
VALUES (1, '2026-05-08', '22:00:00', 360, 6, 0);

INSERT INTO SOMMEIL (Usrid, Somdate, Somheure, Somduree, Somqualite, Somrestful)
VALUES (1, '2026-05-07', '23:15:00', 480, 8, 1);

-- =====================================================================
--                      NUTRITION DOMAIN
-- =====================================================================

-- Enregistrer des repas
INSERT INTO REPAS (Usrid, Repdate, Repheure, Repnom, Repcalories, Repproteines, Repglucides, Replipides)
VALUES (1, CURDATE(), '08:00:00', 'Petit déjeuner', 400, 15, 50, 12);

INSERT INTO REPAS (Usrid, Repdate, Repheure, Repnom, Repcalories, Repproteines, Repglucides, Replipides)
VALUES (1, CURDATE(), '12:30:00', 'Déjeuner', 650, 40, 65, 20);

INSERT INTO REPAS (Usrid, Repdate, Repheure, Repnom, Repcalories, Repproteines, Repglucides, Replipides)
VALUES (1, CURDATE(), '19:00:00', 'Dîner', 550, 35, 55, 18);

INSERT INTO REPAS (Usrid, Repdate, Repheure, Repnom, Repcalories, Repproteines, Repglucides, Replipides)
VALUES (1, CURDATE(), '15:00:00', 'Snack', 150, 5, 20, 5);

-- Repas des jours précédents
INSERT INTO REPAS (Usrid, Repdate, Repheure, Repnom, Repcalories, Repproteines, Repglucides, Replipides)
VALUES (1, '2026-05-09', '08:00:00', 'Petit déjeuner', 420, 16, 52, 13);

INSERT INTO REPAS (Usrid, Repdate, Repheure, Repnom, Repcalories, Repproteines, Repglucides, Replipides)
VALUES (1, '2026-05-09', '12:30:00', 'Déjeuner', 680, 42, 68, 21);

INSERT INTO REPAS (Usrid, Repdate, Repheure, Repnom, Repcalories, Repproteines, Repglucides, Replipides)
VALUES (1, '2026-05-09', '19:00:00', 'Dîner', 580, 36, 58, 19);

-- =====================================================================
--                    BODY MEASUREMENTS DOMAIN
-- =====================================================================

-- Enregistrer des mesures corporelles
INSERT INTO MESURE_CORPS (Usrid, Mesdate, Mespoids, Mestaille, Mestour_poitrine, Mestour_taille, Mestour_hanche)
VALUES (1, CURDATE(), 75.5, 180, 98, 85, 92);

INSERT INTO MESURE_CORPS (Usrid, Mesdate, Mespoids, Mestaille, Mestour_poitrine, Mestour_taille, Mestour_hanche)
VALUES (1, '2026-05-01', 75.8, 180, 99, 86, 93);

INSERT INTO MESURE_CORPS (Usrid, Mesdate, Mespoids, Mestaille, Mestour_poitrine, Mestour_taille, Mestour_hanche)
VALUES (1, '2026-04-24', 76.0, 180, 99, 86, 93);

INSERT INTO MESURE_CORPS (Usrid, Mesdate, Mespoids, Mestaille, Mestour_poitrine, Mestour_taille, Mestour_hanche)
VALUES (1, '2026-04-17', 76.2, 180, 100, 87, 94);

-- =====================================================================
--                         SPORT DOMAIN
-- =====================================================================

-- Créer les types de sports
INSERT INTO TYPE_SPORT (Typnom)
VALUES ('Course');

INSERT INTO TYPE_SPORT (Typnom)
VALUES ('Musculation');

INSERT INTO TYPE_SPORT (Typnom)
VALUES ('Natation');

INSERT INTO TYPE_SPORT (Typnom)
VALUES ('Yoga');

INSERT INTO TYPE_SPORT (Typnom)
VALUES ('Cyclisme');

INSERT INTO TYPE_SPORT (Typnom)
VALUES ('Tennis');

-- Enregistrer des séances de sport
INSERT INTO SEANCE_SPORT (Usrid, Typid, Seadate, Seaheure, Seaduree, Seacalories, Seaintensity)
VALUES (1, 1, CURDATE(), '07:00:00', 45, 450, 'HAUTE');

INSERT INTO SEANCE_SPORT (Usrid, Typid, Seadate, Seaheure, Seaduree, Seacalories, Seaintensity)
VALUES (1, 2, '2026-05-09', '18:00:00', 60, 500, 'HAUTE');

INSERT INTO SEANCE_SPORT (Usrid, Typid, Seadate, Seaheure, Seaduree, Seacalories, Seaintensity)
VALUES (1, 3, '2026-05-08', '10:00:00', 50, 400, 'MOYENNE');

INSERT INTO SEANCE_SPORT (Usrid, Typid, Seadate, Seaheure, Seaduree, Seacalories, Seaintensity)
VALUES (1, 4, '2026-05-07', '19:00:00', 30, 150, 'BASSE');

INSERT INTO SEANCE_SPORT (Usrid, Typid, Seadate, Seaheure, Seaduree, Seacalories, Seaintensity)
VALUES (1, 5, '2026-05-06', '17:00:00', 60, 480, 'MOYENNE');

-- =====================================================================
--                      BREATHING DOMAIN
-- =====================================================================

-- Enregistrer des sessions de respiration cohérence
INSERT INTO SEANCE_COHERENCE (Usrid, Secdate, Secheure, Secduree, Secfrequence)
VALUES (1, CURDATE(), '12:00:00', 5, 'NORMALE');

INSERT INTO SEANCE_COHERENCE (Usrid, Secdate, Secheure, Secduree, Secfrequence)
VALUES (1, '2026-05-09', '12:15:00', 5, 'NORMALE');

INSERT INTO SEANCE_COHERENCE (Usrid, Secdate, Secheure, Secduree, Secfrequence)
VALUES (1, '2026-05-08', '14:00:00', 10, 'RAPIDE');

INSERT INTO SEANCE_COHERENCE (Usrid, Secdate, Secheure, Secduree, Secfrequence)
VALUES (1, '2026-05-07', '20:00:00', 5, 'NORMALE');

-- =====================================================================
--                        ALCOHOL DOMAIN
-- =====================================================================

-- Enregistrer une consommation d'alcool
INSERT INTO CONSOMMATION_ALCOOL (Usrid, Condate, Conheure, Contype, Conquantite, Conforcentage)
VALUES (1, '2026-05-09', '20:00:00', 'Verre de vin', 150, 12.5);

INSERT INTO CONSOMMATION_ALCOOL (Usrid, Condate, Conheure, Contype, Conquantite, Conforcentage)
VALUES (1, '2026-05-09', '21:00:00', 'Bière', 330, 5.0);

INSERT INTO CONSOMMATION_ALCOOL (Usrid, Condate, Conheure, Contype, Conquantite, Conforcentage)
VALUES (1, '2026-05-02', '19:30:00', 'Verre de vin', 150, 12.5);

-- =====================================================================
--                          TODO DOMAIN
-- =====================================================================

-- Ajouter des éléments à faire
INSERT INTO TODO (Usrid, Todtitre, Toddescription, Toddone, Toddate)
VALUES (1, 'Faire la lessive', 'Laver les vêtements', 0, '2026-05-10');

INSERT INTO TODO (Usrid, Todtitre, Toddescription, Toddone, Toddate)
VALUES (1, 'Appeler le plombier', 'Réparer la fuite', 0, '2026-05-11');

INSERT INTO TODO (Usrid, Todtitre, Toddescription, Toddone, Toddate)
VALUES (1, 'Faire les courses', 'Aller au marché', 1, '2026-05-10');

INSERT INTO TODO (Usrid, Todtitre, Toddescription, Toddone, Toddate)
VALUES (1, 'Payer les factures', 'Paiement en ligne', 0, '2026-05-15');

-- =====================================================================
--                     VERIFICATION DES INSERTS
-- =====================================================================

-- Vérifier les comptes créés
-- SELECT * FROM COMPTE WHERE Usrid = 1;

-- Vérifier les transactions
-- SELECT * FROM MOUVEMENT WHERE Usrid = 1 ORDER BY Moudate DESC;

-- Vérifier les habitudes
-- SELECT * FROM HABITUDE WHERE Usrid = 1;

-- Vérifier les repas d'aujourd'hui
-- SELECT * FROM REPAS WHERE Usrid = 1 AND Repdate = CURDATE();

-- Vérifier les séances de sport
-- SELECT * FROM SEANCE_SPORT WHERE Usrid = 1 ORDER BY Seadate DESC;

-- Vérifier l'hydratation aujourd'hui
-- SELECT SUM(Hydquantite) AS total_eau FROM HYDRATATION WHERE Usrid = 1 AND Hyddate = CURDATE();

-- Vérifier les mesures corporelles
-- SELECT * FROM MESURE_CORPS WHERE Usrid = 1 ORDER BY Mesdate DESC;
