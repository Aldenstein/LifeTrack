use std::env;

/// Configuration globale de l'application
/// Contient les paramètres de connexion à la base de données et les secrets JWT
pub struct DbConfig {
    /// URL de connexion à la base de données MySQL/MariaDB
    pub url: String,
    /// Clé secrète pour signer et vérifier les tokens JWT
    pub jwt_secret: String,
}

/// Charge la configuration depuis les variables d'environnement
/// Lit le fichier .env et récupère DATABASE_URL et JWT_SECRET
pub fn load_config() -> DbConfig {
    // Charger le fichier .env s'il existe
    let _ = dotenvy::from_filename("../.env");

    // Récupérer l'URL de la base de données (obligatoire)
    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    // Récupérer le secret JWT (optionnel, utilise une valeur par défaut en dev)
    let jwt_secret = env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-me-in-production".to_string());
    
    DbConfig { url, jwt_secret }
}