import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type {
  ApiAccount,
  ApiFinanceType,
  ApiTransaction,
  ApiPlannedExpense,
} from '@/types/api'

interface FinanceState {
  accounts:        ApiAccount[]
  financeTypes:    ApiFinanceType[]
  transactions:    ApiTransaction[]
  plannedExpenses: ApiPlannedExpense[]

  // Chargement bulk depuis l'API (appele au bootstrap)
  setAccounts:        (data: ApiAccount[]) => void
  setFinanceTypes:    (data: ApiFinanceType[]) => void
  setTransactions:    (data: ApiTransaction[]) => void
  setPlannedExpenses: (data: ApiPlannedExpense[]) => void

  // Ajout unitaire apres POST reussi
  addAccount:        (a: ApiAccount) => void
  addFinanceType:    (t: ApiFinanceType) => void
  addTransaction:    (t: ApiTransaction) => void
  addPlannedExpense: (e: ApiPlannedExpense) => void

  // Suppression locale uniquement (pas de route DELETE dans l'API)
  deleteAccount:        (id: number) => void
  deleteTransaction:    (id: number) => void
  deletePlannedExpense: (id: number) => void

  reset: () => void
}

const initialState = {
  accounts:        [],
  financeTypes:    [],
  transactions:    [],
  plannedExpenses: [],
}

export const useFinanceStore = create<FinanceState>()(
  persist(
    (set) => ({
      ...initialState,

      setAccounts:        (data) => set({ accounts: data }),
      setFinanceTypes:    (data) => set({ financeTypes: data }),
      setTransactions:    (data) => set({ transactions: data }),
      setPlannedExpenses: (data) => set({ plannedExpenses: data }),

      addAccount:        (a) => set((s) => ({ accounts:        [...s.accounts,        a] })),
      addFinanceType:    (t) => set((s) => ({ financeTypes:    [...s.financeTypes,    t] })),
      addTransaction:    (t) => set((s) => ({ transactions:    [...s.transactions,    t] })),
      addPlannedExpense: (e) => set((s) => ({ plannedExpenses: [...s.plannedExpenses, e] })),

      // Suppression locale — cascade sur transactions et depenses planifiees liees
      deleteAccount: (id) => set((s) => ({
        accounts:        s.accounts.filter((a) => a.id !== id),
        transactions:    s.transactions.filter((t) => t.account_id !== id),
        plannedExpenses: s.plannedExpenses.filter((e) => e.account_id !== id),
      })),

      deleteTransaction: (id) => set((s) => ({
        transactions: s.transactions.filter((t) => t.id !== id),
      })),

      deletePlannedExpense: (id) => set((s) => ({
        plannedExpenses: s.plannedExpenses.filter((e) => e.id !== id),
      })),

      reset: () => set(initialState),
    }),
    { name: 'finance-storage' }
  )
)
