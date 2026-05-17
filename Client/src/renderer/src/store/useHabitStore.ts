import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { Habit, HabitEntry } from '@/types'
import { habitService } from '@/services/habitService'
import { useUserStore } from './userStore'

interface HabitState {
  habits: Habit[]
  setHabits: (habits: Habit[]) => void
  addHabit: (habit: Habit) => Promise<void>
  updateHabit: (id: string, updates: Partial<Habit>) => Promise<void>
  deleteHabit: (id: string) => Promise<void>
  logEntry: (habitId: string, entry: HabitEntry) => Promise<void>
  reset: () => void
}

const initialState = { habits: [] }

export const useHabitStore = create<HabitState>()(
  persist(
    (set) => ({
      ...initialState,

      setHabits: (habits) => set({ habits }),

      addHabit: async (habit) => {
        const userId = useUserStore.getState().profile?.id
        if (!userId) return
        const created = await habitService.createHabit(userId, habit)
        set((state) => ({ habits: [...state.habits, created] }))
      },

      updateHabit: async (id, updates) => {
        const userId = useUserStore.getState().profile?.id
        if (!userId) return
        const updated = await habitService.updateHabit(userId, id, updates)
        set((state) => ({ habits: state.habits.map((h) => (h.id === id ? updated : h)) }))
      },

      deleteHabit: async (id) => {
        const userId = useUserStore.getState().profile?.id
        if (!userId) return
        await habitService.deleteHabit(userId, id)
        set((state) => ({ habits: state.habits.filter((h) => h.id !== id) }))
      },

      logEntry: async (habitId, entry) => {
        const userId = useUserStore.getState().profile?.id
        if (!userId) return
        const updated = await habitService.logEntry(userId, habitId, entry)
        set((state) => ({ habits: state.habits.map((h) => (h.id === habitId ? updated : h)) }))
      },

      reset: () => set(initialState),
    }),
    { name: 'habit-storage' }
  )
)