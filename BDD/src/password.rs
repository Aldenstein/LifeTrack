use argon2::{
    password_hash::{SaltString, PasswordHasher, PasswordHash, PasswordVerifier},
    Argon2,
};
use rand::Rng;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use hex;

/// Hash une passphrase avec Argon2
/// Argon2 est résistant aux attaques brute-force (défend contre GPU/ASIC)
pub fn hash_passphrase(passphrase: &str) -> Result<String, String> {
    let salt = SaltString::generate(rand::thread_rng());
    let argon2 = Argon2::default();

    argon2
        .hash_password(passphrase.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| format!("Failed to hash passphrase: {}", e))
}

/// Vérifie si une passphrase correspond à son hash Argon2
/// Retourne true si valide, false sinon
pub fn verify_passphrase(passphrase: &str, hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| format!("Failed to parse hash: {}", e))?;
    let argon2 = Argon2::default();

    argon2
        .verify_password(passphrase.as_bytes(), &parsed_hash)
        .map(|_| true)
        .or_else(|_| Ok(false))
}

/// Dérive une clé de chiffrement AES-256 (32 bytes) à partir de la passphrase
/// Utilise PBKDF2-HMAC-SHA256 avec 100 000 itérations (recommandation OWASP 2024)
/// La clé est déterministe : même passphrase + même salt => même clé
pub fn derive_encryption_key(passphrase: &str, salt: &[u8]) -> String {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(passphrase.as_bytes(), salt, 100_000, &mut key);
    hex::encode(key)
}

/// Génère un salt aléatoire sécurisé (16 bytes)
/// Utilisé UNE SEULE FOIS à la création du compte, puis persisté en base
pub fn generate_salt() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut salt = vec![0u8; 16];
    rng.fill(&mut salt[..]);
    salt
}
