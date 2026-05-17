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

  // Chargement depuis l'API
  setAccounts:        (data: ApiAccount[])        => void
  setFinanceTypes:    (data: ApiFinanceType[])     => void
  setTransactions:    (data: ApiTransaction[])     => void
  setPlannedExpenses: (data: ApiPlannedExpense[])  => void

  // Ajout après POST réussi
  addAccount:        (a: ApiAccount)        => void
  addFinanceType:    (t: ApiFinanceType)     => void
  addTransaction:    (t: ApiTransaction)     => void
  addPlannedExpense: (e: ApiPlannedExpense)  => void

  // Reset (déconnexion)
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

      // Setters bulk (GET au démarrage)
      setAccounts:        (data) => set({ accounts: data }),
      setFinanceTypes:    (data) => set({ financeTypes: data }),
      setTransactions:    (data) => set({ transactions: data }),
      setPlannedExpenses: (data) => set({ plannedExpenses: data }),

      // Ajout unitaire (après POST)
      addAccount:        (a) => set(s => ({ accounts:        [...s.accounts,        a] })),
      addFinanceType:    (t) => set(s => ({ financeTypes:    [...s.financeTypes,    t] })),
      addTransaction:    (t) => set(s => ({ transactions:    [...s.transactions,    t] })),
      addPlannedExpense: (e) => set(s => ({ plannedExpenses: [...s.plannedExpenses, e] })),

      reset: () => set(initialState),
    }),
    { name: 'finance-storage' }
  )
)