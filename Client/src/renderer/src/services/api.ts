import { useUserStore } from '@/store/userStore'

// ✅ Base URL — pointe vers le tunnel Cloudflare en prod
// En dev local, change dans .env : VITE_API_URL=http://127.0.0.1:3000
const BASE_URL = import.meta.env.VITE_API_URL ?? 'https://lifetrack.chocsathan.fr'

function getToken(): string {
  const token = useUserStore.getState().token
  if (!token) throw new Error('Non authentifié')
  return token
}

async function request<T>(
  method: string,
  path: string,
  body?: unknown,
  requiresAuth = true
): Promise<T> {
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
  }

  // ✅ Auth optionnelle — les routes publiques (/auth/*) passent requiresAuth=false
  if (requiresAuth) {
    headers['Authorization'] = `Bearer ${getToken()}`
  }

  let res: Response
  try {
    res = await fetch(`${BASE_URL}${path}`, {
      method,
      headers,
      body: body !== undefined ? JSON.stringify(body) : undefined,
    })
  } catch (networkError: any) {
    throw new Error(
      `Erreur réseau vers ${BASE_URL}${path} : ${networkError?.message ?? 'connexion impossible'}`
    )
  }

  // ✅ Gestion du 401 — token expiré ou invalide (JWT dure 7 jours)
  if (res.status === 401) {
    throw new Error('Session expirée — veuillez vous reconnecter')
  }

  if (!res.ok) {
    const bodyText = await res.text().catch(() => '')
    const errorMessage = bodyText || `${res.status} ${res.statusText}`
    console.error('API request failed', {
      method,
      path,
      status: res.status,
      statusText: res.statusText,
      bodyText: bodyText.slice(0, 100),
    })
    throw new Error(`Erreur API ${res.status} : ${errorMessage}`)
  }

  const text = await res.text()
  return text ? JSON.parse(text) : ({} as T)
}

// ── Exports ──────────────────────────────────────────────────────────────────

export async function getJson<T>(path: string): Promise<T> {
  return request<T>('GET', path)
}

export async function postJson<T>(path: string, body: unknown, requiresAuth = true): Promise<T> {
  return request<T>('POST', path, body, requiresAuth)
}

export async function putJson<T>(path: string, body: unknown): Promise<T> {
  return request<T>('PUT', path, body)
}

export async function patchJson<T>(path: string, body: unknown): Promise<T> {
  return request<T>('PATCH', path, body)
}

export async function delJson<T>(path: string): Promise<T> {
  return request<T>('DELETE', path)
}
