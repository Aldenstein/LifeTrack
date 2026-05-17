import { useState } from 'react'
import { useUserStore }    from '@/store/userStore'
import { useFinanceStore } from '@/store/useFinanceStore'
import { authService }     from '@/services/authService'
import type { LoginRequest, RegisterRequest } from '@/types/api'

export function useAuth() {
  const { setToken, setProfile, reset: resetUser }       = useUserStore()
  const { reset: resetFinance }                          = useFinanceStore()
  const [loading, setLoading] = useState(false)
  const [error,   setError]   = useState<string | null>(null)

  async function login(body: LoginRequest) {
    try {
      setLoading(true); setError(null)
      const { token } = await authService.login(body)
      setToken(token)
      const profile   = await authService.getProfile(token)
      setProfile(profile)
      return true
    } catch (err: any) {
      setError(err.message)
      return false
    } finally {
      setLoading(false)
    }
  }

  async function register(body: RegisterRequest) {
    try {
      setLoading(true); setError(null)
      const { token } = await authService.register(body)
      setToken(token)
      const profile   = await authService.getProfile(token)
      setProfile(profile)
      return true
    } catch (err: any) {
      setError(err.message)
      return false
    } finally {
      setLoading(false)
    }
  }

  function logout() {
    resetUser()
    resetFinance()
  }

  return { login, register, logout, loading, error }
}
