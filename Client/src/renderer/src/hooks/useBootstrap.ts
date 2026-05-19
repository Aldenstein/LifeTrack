import { useEffect, useState } from 'react'
import { useUserStore }    from '@/store/userStore'
import { useFinanceStore } from '@/store/useFinanceStore'
import { useHabitStore }   from '@/store/useHabitStore'
import { useTodoStore }    from '@/store/useTodoStore'
import { authService }     from '@/services/authService'
import { financeService }  from '@/services/financeService'
import { habitService }    from '@/services/habitService'
import { todoService }     from '@/services/todoService'

export function useBootstrap() {
  const { token, setToken, setProfile, reset: resetUser } = useUserStore()
  const { setAccounts, setFinanceTypes, setTransactions, setPlannedExpenses, reset: resetFinance } = useFinanceStore()
  const { setHabits, loadCounters, reset: resetHabits } = useHabitStore()
  const { setTodos, reset: resetTodos } = useTodoStore()

  const [ready, setReady] = useState(false)

  useEffect(() => {
    let cancelled = false

    if (!token) {
      resetUser(); resetFinance(); resetHabits(); resetTodos()
      setReady(true)
      return
    }

    async function boot() {
      try {
        const storedProfile = useUserStore.getState().profile
        if (!storedProfile?.id) throw new Error('Profil introuvable')

        const profile = await authService.getProfile(storedProfile.id)
        if (cancelled) return
        setToken(token!)
        setProfile(profile)

        const [accounts, types, transactions, planned] = await Promise.all([
          financeService.getAccounts(profile.id),
          financeService.getFinanceTypes(),
          // Utilise le endpoint /period qui existe dans l'API
          financeService.getTransactions(profile.id).catch(() => useFinanceStore.getState().transactions),
          financeService.getPlannedExpenses(profile.id),
        ])

        const [habits, todosFromApi] = await Promise.all([
          habitService.getHabitsActive(profile.id),
          // GET /todos n'existe pas dans l'API — retourne [] sans erreur
          todoService.getTodos(profile.id),
        ])

        if (cancelled) return
        setAccounts(accounts)
        setFinanceTypes(types)
        setTransactions(transactions)
        setPlannedExpenses(planned)
        setHabits(habits)
        // Ne remplace les todos locaux que si l'API en a retourné
        if (todosFromApi.length > 0) setTodos(todosFromApi)

        await loadCounters()

        if (!cancelled) setReady(true)
      } catch {
        if (cancelled) return
        resetUser(); resetFinance(); resetHabits(); resetTodos()
        setReady(true)
      }
    }

    boot()
    return () => { cancelled = true }
  }, [token])

  return { ready }
}
