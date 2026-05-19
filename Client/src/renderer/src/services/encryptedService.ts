// services/encryptedService.ts
// Service pour les routes Zero-Knowledge de l'API
// POST/GET /users/{user_id}/encrypted
// Le serveur ne voit jamais le contenu en clair

import { getJson, postJson } from './api'

export interface EncryptedEntry {
  id?: number
  date: string
  iv: string
  ciphertext: string
  version: number
  // tag optionnel pour filtrer par type de donnee (ex: 'habit-counter')
  tag?: string
}

export const encryptedService = {
  // POST /users/{user_id}/encrypted — sauvegarder une entree chiffree
  save: (userId: number, entry: Omit<EncryptedEntry, 'id'>) =>
    postJson<EncryptedEntry>(`/users/${userId}/encrypted`, entry),

  // GET /users/{user_id}/encrypted/all — recuperer toutes les entrees chiffrees
  getAll: (userId: number) =>
    getJson<EncryptedEntry[]>(`/users/${userId}/encrypted/all`),
}
