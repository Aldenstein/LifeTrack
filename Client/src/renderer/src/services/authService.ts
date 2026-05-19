import type { LoginRequest, RegisterRequest, AuthResponse } from '@/types/api'
import { getJson } from './api'

const BASE_URL = import.meta.env.VITE_API_URL ?? 'https://lifetrack.chocsathan.fr'

// ✅ Appel sans token — utilisé uniquement pour les routes publiques /auth/*
async function postPublic<T>(path: string, body: unknown): Promise<T> {
  const res = await fetch(`${BASE_URL}${path}`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(body),
  })
  if (!res.ok) {
    const msg = await res.text().catch(() => 'Erreur serveur')
    throw new Error(msg || `HTTP ${res.status}`)
  }
  return res.json()
}

export const authService = {
  // POST /auth/login → retourne { token, user_id, encryption_key, encryption_salt }
  login: (body: LoginRequest) =>
    postPublic<AuthResponse>('/auth/login', body),

  // POST /auth/register → retourne { token, user_id, encryption_key, encryption_salt }
  // ⚠️  Stocker immédiatement encryption_key + encryption_salt côté client
  //     Si perdus, les données Zero-Knowledge ne peuvent plus être déchiffrées
  register: (body: RegisterRequest) =>
    postPublic<AuthResponse>('/auth/register', body),

  // POST /auth/derive-keys → retourne { encryption_key, salt }
  // ✅ Utile si l'utilisateur réinstalle l'app et perd son localStorage
  //    Ne génère PAS de nouveau token JWT
  deriveKeys: (body: { email: string; passphrase: string }) =>
    postPublic<{ encryption_key: string; salt: string }>('/auth/derive-keys', body),

  // GET /users/me/{user_id} → profil de l'utilisateur connecté
  // ✅ Utilise le client partagé (lit le token depuis le store)
  getProfile: (userId: number) =>
    getJson<any>(`/users/me/${userId}`),
}
