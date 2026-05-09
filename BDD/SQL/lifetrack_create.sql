-- -----------------------------------------------------------
-- 0. UTILISATEUR
-- -----------------------------------------------------------

CREATE TABLE UTILISATEUR (
    Usrid         INT          NOT NULL AUTO_INCREMENT,
    UsrpublicId   CHAR(64)     NOT NULL,
    UsrcreatedAt  DATETIME     NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (Usrid),
    UNIQUE (UsrpublicId)
);

-- -----------------------------------------------------------
-- 1. FINANCES
-- -----------------------------------------------------------

CREATE TABLE TYPE (
    Typid   INT          NOT NULL AUTO_INCREMENT,
    Typtitre VARCHAR(255) NOT NULL,
    PRIMARY KEY (Typid)
);

CREATE TABLE COMPTE (
    Comid     INT    NOT NULL AUTO_INCREMENT,
    Comnom    VARCHAR(255) NOT NULL,
    Comsolde  DOUBLE NOT NULL,
    Usrid     INT    NOT NULL,
    PRIMARY KEY (Comid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

CREATE TABLE MOUVEMENT (
    Mouid      INT    NOT NULL AUTO_INCREMENT,
    Moumontant DOUBLE NOT NULL,
    Moudate    DATE   NOT NULL,
    Typid      INT,
    Comid      INT,
    Usrid      INT    NOT NULL,
    PRIMARY KEY (Mouid),
    FOREIGN KEY (Typid) REFERENCES TYPE(Typid),
    FOREIGN KEY (Comid) REFERENCES COMPTE(Comid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

CREATE TABLE FACTURE (
    Facid             INT  NOT NULL AUTO_INCREMENT,
    Facdate           DATE NOT NULL,
    Facperiodicite    ENUM('JOUR','SEMAINE','MOIS','ANNEE') NOT NULL,
    Facintervalle     INT  NOT NULL,
    FacdateProchain   DATE,          -- CALCULÉ côté applicatif
    Facdone           TINYINT(1),    -- CALCULÉ côté applicatif
    Mouid             INT           NOT NULL,
    Typid             INT,
    Comid             INT,
    Usrid             INT  NOT NULL,
    PRIMARY KEY (Facid),
    UNIQUE (Mouid),
    FOREIGN KEY (Mouid) REFERENCES MOUVEMENT(Mouid),
    FOREIGN KEY (Typid) REFERENCES TYPE(Typid),
    FOREIGN KEY (Comid) REFERENCES COMPTE(Comid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

-- -----------------------------------------------------------
-- 2. HABITUDES
-- -----------------------------------------------------------

CREATE TABLE CATEGORIE (
    Catid   INT          NOT NULL AUTO_INCREMENT,
    Catnom  VARCHAR(255) NOT NULL,
    Catplus ENUM('1','-1') NOT NULL,
    PRIMARY KEY (Catid)
);

CREATE TABLE HABITUDE (
    Habid  INT          NOT NULL AUTO_INCREMENT,
    Habnom VARCHAR(500) NOT NULL,
    Catid  INT,
    Usrid  INT          NOT NULL,
    PRIMARY KEY (Habid),
    FOREIGN KEY (Catid) REFERENCES CATEGORIE(Catid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

-- -----------------------------------------------------------
-- 3. HUMEUR  (déclarée avant BILAN qui la référence)
-- -----------------------------------------------------------

CREATE TABLE HUMEUR (
    Humid    INT          NOT NULL AUTO_INCREMENT,
    Humnom   VARCHAR(255) NOT NULL,
    Humcolor VARCHAR(8)   NOT NULL,
    PRIMARY KEY (Humid)
);

CREATE TABLE DATE_HUMEUR (
    Usrid  INT  NOT NULL,
    DHdate DATE NOT NULL,
    Humid  INT,
    PRIMARY KEY (Usrid, DHdate),
    FOREIGN KEY (Humid) REFERENCES HUMEUR(Humid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

-- -----------------------------------------------------------
-- 2. HABITUDES (suite — BILAN référence HUMEUR)
-- -----------------------------------------------------------

CREATE TABLE BILAN (
    Bilid   INT  NOT NULL AUTO_INCREMENT,
    Bildate DATE NOT NULL,
    Humid   INT,
    Usrid   INT  NOT NULL,
    PRIMARY KEY (Bilid),
    FOREIGN KEY (Humid) REFERENCES HUMEUR(Humid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

CREATE TABLE HABITUDE_BILAN (
    Bilid  INT         NOT NULL,
    Habid  INT         NOT NULL,
    HBdone TINYINT(1)  NOT NULL,
    PRIMARY KEY (Bilid, Habid),
    FOREIGN KEY (Bilid) REFERENCES BILAN(Bilid),
    FOREIGN KEY (Habid) REFERENCES HABITUDE(Habid)
);

-- -----------------------------------------------------------
-- 4. HYDRATATION
-- -----------------------------------------------------------

CREATE TABLE HYDRATATION (
    Hydid        INT  NOT NULL AUTO_INCREMENT,
    Hyddate      DATE NOT NULL,
    Hydquantite  INT  NOT NULL,
    Hydobjectif  INT  NOT NULL,
    Usrid        INT  NOT NULL,
    PRIMARY KEY (Hydid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

-- -----------------------------------------------------------
-- 5. SOMMEIL
-- -----------------------------------------------------------

CREATE TABLE SOMMEIL (
    Somid        INT        NOT NULL AUTO_INCREMENT,
    Somdate      DATE       NOT NULL,
    Somcoucher   TIME       NOT NULL,
    Somlever     TIME       NOT NULL,
    Somduree     INT,                 -- CALCULÉ côté applicatif
    Somreposant  TINYINT(1),          -- OPTIONNEL
    Usrid        INT        NOT NULL,
    PRIMARY KEY (Somid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

-- -----------------------------------------------------------
-- 6. REPAS / NUTRITION
-- -----------------------------------------------------------

CREATE TABLE REPAS (
    Repid           INT    NOT NULL AUTO_INCREMENT,
    Repdate         DATE   NOT NULL,
    Repdescription  TEXT,
    Repcalories     DOUBLE,
    Repproteines    DOUBLE,
    Repglucides     DOUBLE,
    Replipides      DOUBLE,
    Usrid           INT    NOT NULL,
    PRIMARY KEY (Repid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

-- -----------------------------------------------------------
-- 7. MESURES CORPORELLES
-- -----------------------------------------------------------

CREATE TABLE MESURE_CORPORELLE (
    Mesid         INT    NOT NULL AUTO_INCREMENT,
    Mesdate       DATE   NOT NULL,
    Mespoids      DOUBLE NOT NULL,
    Mestaille     DOUBLE,             -- OPTIONNEL
    MesIMC        DOUBLE,             -- CALCULÉ côté applicatif
    MesMetaBasal  DOUBLE,             -- CALCULÉ côté applicatif
    Usrid         INT    NOT NULL,
    PRIMARY KEY (Mesid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

-- -----------------------------------------------------------
-- 8. SPORT
-- -----------------------------------------------------------

CREATE TABLE SPORT_TYPE (
    Stypid  INT          NOT NULL AUTO_INCREMENT,
    Stypnom VARCHAR(255) NOT NULL,
    PRIMARY KEY (Stypid)
);

CREATE TABLE SEANCE_SPORT (
    Seaid         INT    NOT NULL AUTO_INCREMENT,
    Seadate       DATE   NOT NULL,
    Stypid        INT,
    Seaduree      INT    NOT NULL,
    Seaintensite  ENUM('FAIBLE','MODERE','INTENSE') NOT NULL,
    Seacalories   DOUBLE,             -- CALCULÉ côté applicatif
    Usrid         INT    NOT NULL,
    PRIMARY KEY (Seaid),
    FOREIGN KEY (Stypid) REFERENCES SPORT_TYPE(Stypid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

-- -----------------------------------------------------------
-- 9. SOBRIÉTÉ & ALCOOL
-- -----------------------------------------------------------

CREATE TABLE SOBRIETE (
    Sobid    INT      NOT NULL AUTO_INCREMENT,
    Sobdebut DATETIME NOT NULL,
    Sobfin   DATETIME,                -- NULLABLE = en cours
    Usrid    INT      NOT NULL,
    PRIMARY KEY (Sobid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

CREATE TABLE CONSOMMATION_ALCOOL (
    Alcid          INT      NOT NULL AUTO_INCREMENT,
    Alcdateheure   DATETIME NOT NULL,
    Alcquantite    DOUBLE   NOT NULL,
    Alcdegre       DOUBLE   NOT NULL,
    Alcjeun        TINYINT(1) NOT NULL,
    Alcalcoolemie  DOUBLE,            -- CALCULÉ côté applicatif
    Alctempsobre   INT,               -- CALCULÉ côté applicatif
    Usrid          INT      NOT NULL,
    PRIMARY KEY (Alcid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

-- -----------------------------------------------------------
-- 10. TODO LIST
-- -----------------------------------------------------------

CREATE TABLE TODO_TYPE (
    Totypid INT          NOT NULL AUTO_INCREMENT,
    Totynom VARCHAR(255) NOT NULL,
    PRIMARY KEY (Totypid)
);

CREATE TABLE TODO (
    Todid    INT          NOT NULL AUTO_INCREMENT,
    Todtitre VARCHAR(500) NOT NULL,
    Toddone  TINYINT(1)   NOT NULL,
    Todtimer INT,                     -- NULLABLE = sans timer
    Totypid  INT,
    Usrid    INT          NOT NULL,
    PRIMARY KEY (Todid),
    FOREIGN KEY (Totypid) REFERENCES TODO_TYPE(Totypid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);

-- -----------------------------------------------------------
-- 11. COHÉRENCE CARDIAQUE
-- -----------------------------------------------------------

CREATE TABLE COHERENCE_CARDIAQUE (
    Cohid          INT      NOT NULL AUTO_INCREMENT,
    Cohdateheure   DATETIME NOT NULL,
    Cohduree       INT      NOT NULL,
    Cohparamcercle JSON,               -- OPTIONNEL
    Usrid          INT      NOT NULL,
    PRIMARY KEY (Cohid),
    FOREIGN KEY (Usrid) REFERENCES UTILISATEUR(Usrid)
);
