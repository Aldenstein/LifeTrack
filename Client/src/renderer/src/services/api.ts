
import { useUserStore } from '@/store/userStore'

const BASE_URL = import.meta.env.VITE_API_URL ?? 'http://localhost:8080'

function getToken(): string {
  const token = useUserStore.getState().token
  if (!token) throw new Error('Non authentifié')
  return token
}

async function request<T>(method: string, path: string, body?: unknown): Promise<T> {
  const headers: Record<string, string> = {
    'Content-Type': 'application/json',
    'Authorization': `Bearer ${getToken()}`,
  }

  const res = await fetch(`${BASE_URL}${path}`, {
    method,
    headers,
    body: body !== undefined ? JSON.stringify(body) : undefined,
  })

  if (!res.ok) {
    const msg = await res.text().catch(() => 'Erreur serveur')
    throw new Error(msg || `HTTP ${res.status}`)
  }

  const text = await res.text()
  return text ? JSON.parse(text) : ({} as T)
}

export async function getJson<T>(path: string): Promise<T> {
  return request<T>('GET', path)
}

export async function postJson<T>(path: string, body: unknown): Promise<T> {
  return request<T>('POST', path, body)
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
