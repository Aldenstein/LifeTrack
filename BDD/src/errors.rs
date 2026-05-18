/// Module de gestion des erreurs
/// Actuellement minimal, peut être étendu pour logging structuré

/// Enregistre une erreur dans les logs
/// TODO: Remplacer par un système de logging professionnel (tracing, sentry)
pub fn log_error() {
    println!("[ERROR] Une erreur est survenue");
}