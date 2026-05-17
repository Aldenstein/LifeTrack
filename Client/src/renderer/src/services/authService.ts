import type { LoginRequest, RegisterRequest, AuthResponse, ApiProfile } from '@/types/api'

const BASE_URL = import.meta.env.VITE_API_URL ?? 'http://localhost:3000'

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

async function getPublicAuth<T>(path: string, token: string): Promise<T> {
  const res = await fetch(`${BASE_URL}${path}`, {
    headers: { Authorization: `Bearer ${token}` },
  })
  if (!res.ok) throw new Error('Session invalide')
  return res.json()
}

export const authService = {
  login:      (body: LoginRequest)    => postPublic<AuthResponse>(  '/auth/login',    body),
  register:   (body: RegisterRequest) => postPublic<AuthResponse>(  '/auth/register', body),
  getProfile: (token: string)         => getPublicAuth<ApiProfile>( '/users/me',      token),
}
