-- Migration pour ajouter l'authentification par passphrase
-- Execute cette migration sur la base de données existante

-- Ajouter les colonnes email et passphrase_hash à la table UTILISATEUR
ALTER TABLE UTILISATEUR
ADD COLUMN email VARCHAR(255) UNIQUE AFTER UsrpublicId,
ADD COLUMN passphrase_hash VARCHAR(255) AFTER email;

-- Index pour email
CREATE INDEX idx_email ON UTILISATEUR(email);
