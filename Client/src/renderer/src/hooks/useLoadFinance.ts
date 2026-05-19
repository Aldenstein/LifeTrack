import { useEffect, useState } from 'react'
import { useFinanceStore } from '@/store/useFinanceStore'
import { financeService } from '@/services/financeService'

export function useLoadFinance(userId: number) {
  const { setAccounts, setFinanceTypes, setTransactions, setPlannedExpenses } = useFinanceStore()
  const [loading, setLoading] = useState(true)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    async function load() {
      try {
        if (!userId) {
          setAccounts([]); setFinanceTypes([])
          setTransactions([]); setPlannedExpenses([])
          setError(null); return
        }
        setLoading(true)
        const [accounts, types, transactions, planned] = await Promise.all([
          financeService.getAccounts(userId),
          financeService.getFinanceTypes(),
          financeService.getTransactions(userId),
          financeService.getPlannedExpenses(userId),
        ])
        setAccounts(accounts); setFinanceTypes(types)
        setTransactions(transactions); setPlannedExpenses(planned)
      } catch (err: any) {
        setError(err?.message ?? 'Impossible de charger les finances')
      } finally {
        setLoading(false)
      }
    }
    load()
  }, [userId]) // ✅ Simplifié — setters Zustand stables, pas besoin de les lister

  return { loading, error }
}
