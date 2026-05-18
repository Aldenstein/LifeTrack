// Module de gestion des erreurs et points d'extension
//
// Description (FR):
// Ce module regroupe les types et utilitaires relatifs au traitement
// des erreurs côté serveur. Pour l'instant il est minimal mais peut
// être étendu pour ajouter du logging structuré, des codes d'erreur
// applicatifs, et des conversions d'erreurs internes vers des réponses
// HTTP standardisées.

// Exemple d'extension possible:
// - définir une enum `AppError` avec variants (DbError, AuthError, BadRequest, ...)
// - implémenter `From<AppError> for (StatusCode, Json<ApiError>)` pour
//   centraliser la conversion vers le format JSON renvoyé aux clients.
