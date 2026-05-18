-- 1. Ajouter la colonne (nullable pour les comptes existants)
ALTER TABLE UTILISATEUR
    ADD COLUMN encryption_salt VARCHAR(32) NULL
    COMMENT 'Salt hex 16 bytes pour derivation PBKDF2-HMAC-SHA256 cle AES-256';

-- 2. Index optionnel (non-unique, juste pour les audits)
-- ALTER TABLE UTILISATEUR ADD INDEX idx_encryption_salt (encryption_salt);

-- 3. Verification
SELECT
    COLUMN_NAME,
    COLUMN_TYPE,
    IS_NULLABLE,
    COLUMN_COMMENT
FROM INFORMATION_SCHEMA.COLUMNS
WHERE TABLE_SCHEMA = DATABASE()
  AND TABLE_NAME   = 'UTILISATEUR'
  AND COLUMN_NAME  = 'encryption_salt';
