import { useState } from 'react'
import { useUserStore } from '@/store/userStore'
import { useFinanceStore } from '@/store/useFinanceStore'
import { useHabitStore } from '@/store/useHabitStore'
import { useTodoStore } from '@/store/useTodoStore'
import { authService } from '@/services/authService'
import type { LoginRequest, RegisterRequest } from '@/types/api'

export function useAuth() {
  const { setToken, setProfile, setEncryption, reset: resetUser } = useUserStore()
  const { reset: resetFinance } = useFinanceStore()
  const { reset: resetHabits } = useHabitStore()
  const { reset: resetTodos } = useTodoStore()

  const [loading, setLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)

  async function login(body: LoginRequest) {
    try {
      setLoading(true)
      setError(null)
      const res = await authService.login(body)

      setToken(res.token)
      setEncryption(res.encryption_key, res.encryption_salt)

      const profile = await authService.getProfile(res.user_id)
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
      setLoading(true)
      setError(null)
      const res = await authService.register(body)

      setToken(res.token)
      setEncryption(res.encryption_key, res.encryption_salt)

      const profile = await authService.getProfile(res.user_id)
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
    resetHabits()
    resetTodos()
  }

  return { login, register, logout, loading, error }
}
