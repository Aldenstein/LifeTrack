import { useEffect, useState } from 'react'
import { useUserStore } from '@/store/userStore'
import { useFinanceStore } from '@/store/useFinanceStore'
import { useHabitStore } from '@/store/useHabitStore'
import { useTodoStore } from '@/store/useTodoStore'
import { authService } from '@/services/authService'
import { financeService } from '@/services/financeService'
import { habitService } from '@/services/habitService'
import { todoService } from '@/services/todoService'

export function useBootstrap() {
  const { token, setToken, setProfile, reset: resetUser } = useUserStore()
  const {
    setAccounts,
    setFinanceTypes,
    setTransactions,
    setPlannedExpenses,
    reset: resetFinance,
  } = useFinanceStore()
  const { setHabits, reset: resetHabits } = useHabitStore()
  const { setTodos, reset: resetTodos } = useTodoStore()

  const [ready, setReady] = useState(false)

  useEffect(() => {
    let cancelled = false

    if (!token) {
      resetUser()
      resetFinance()
      resetHabits()
      resetTodos()
      setReady(true)
      return
    }

    async function boot() {
      try {
        const profile = await authService.getProfile(token!)
        if (cancelled) return
        setToken(token!)
        setProfile(profile)

        const [accounts, types, transactions, planned] = await Promise.all([
          financeService.getAccounts(profile.id),
          financeService.getFinanceTypes(),
          financeService.getTransactions(profile.id),
          financeService.getPlannedExpenses(profile.id),
        ])
        const [habits, todos] = await Promise.all([
          habitService.getHabits(profile.id),
          todoService.getTodos(profile.id),
        ])
        if (cancelled) return
        setAccounts(accounts)
        setFinanceTypes(types)
        setTransactions(transactions)
        setPlannedExpenses(planned)
        setHabits(habits)
        setTodos(todos)
        if (!cancelled) setReady(true)
      } catch {
        if (cancelled) return
        resetUser()
        resetFinance()
        resetHabits()
        resetTodos()
        setReady(true)
      }
    }

    boot()
    return () => {
      cancelled = true
    }
  }, [token, resetFinance, resetHabits, resetTodos, resetUser, setAccounts, setFinanceTypes, setPlannedExpenses, setProfile, setToken, setTransactions, setHabits, setTodos])

  return { ready }
}