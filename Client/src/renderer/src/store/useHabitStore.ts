import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { Habit } from '@/types'
import { habitService }    from '@/services/habitService'
import { encryptedService } from '@/services/encryptedService'
import { encryptData, decryptData } from '@/utils/encryption'
import { useUserStore }    from './userStore'

interface CounterPayload {
  habitId: string
  date:    string
  value:   number
}

interface HabitState {
  habits: Habit[]
  setHabits:      (habits: Habit[]) => void
  addHabit:       (habit: Habit) => Promise<void>
  updateHabit:    (id: string, updates: Partial<Habit>) => Promise<void>
  deleteHabit:    (id: string) => Promise<void>
  completeHabit:  (habitId: string, date: string) => Promise<void>
  updateCounter:  (habitId: string, date: string, value: number) => Promise<void>
  loadCounters:   () => Promise<void>
  reset: () => void
}

const initialState = { habits: [] }

export const useHabitStore = create<HabitState>()(
  persist(
    (set, get) => ({
      ...initialState,

      setHabits: (habits) => set({ habits }),

      addHabit: async (habit) => {
        const userId = useUserStore.getState().profile?.id
        if (!userId) return
        try {
          const created = await habitService.createHabit(userId, habit)
          set((state) => ({ habits: [...state.habits, created] }))
        } catch {
          set((state) => ({ habits: [...state.habits, habit] }))
        }
      },

      // Pas de route PUT dans l'API — mise à jour locale uniquement
      updateHabit: async (id, updates) => {
        set((state) => ({
          habits: state.habits.map((h) => h.id === id ? { ...h, ...updates } : h),
        }))
      },

      // Pas de route DELETE dans l'API — suppression locale uniquement
      deleteHabit: async (id) => {
        set((state) => ({ habits: state.habits.filter((h) => h.id !== id) }))
      },

      completeHabit: async (habitId, date) => {
        const userId = useUserStore.getState().profile?.id
        if (!userId) return
        try {
          await habitService.completeHabit(userId, habitId, date)
          const todayHabits = await habitService.getHabitsToday(userId)
          set({ habits: todayHabits })
        } catch {
          // Optimistic local update si l'API échoue
          set((state) => ({
            habits: state.habits.map((h) => {
              if (h.id !== habitId) return h
              const entries = h.entries.filter((e) => e.date !== date)
              return { ...h, entries: [...entries, { date, value: 1 }] }
            }),
          }))
        }
      },

      updateCounter: async (habitId, date, value) => {
        const { profile, encryptionKey } = useUserStore.getState()
        if (!profile?.id || !encryptionKey) return

        set((state) => ({
          habits: state.habits.map((h) => {
            if (h.id !== habitId) return h
            const entries = h.entries.filter((e) => e.date !== date)
            return { ...h, entries: [...entries, { date, value }] }
          }),
        }))

        const payload: CounterPayload = { habitId, date, value }
        const { iv, ciphertext } = await encryptData(payload, encryptionKey)

        await encryptedService.save(profile.id, {
          date,
          iv,
          ciphertext,
          version: 1,
          tag: 'habit-counter',
        })

        const habit = get().habits.find((h) => h.id === habitId)
        if (habit && value >= (habit.goal ?? 100)) {
          await habitService.completeHabit(profile.id, habitId, date)
        }
      },

      loadCounters: async () => {
        const { profile, encryptionKey } = useUserStore.getState()
        if (!profile?.id || !encryptionKey) return

        const entries = await encryptedService.getAll(profile.id)
        const counterEntries = entries.filter((e) => (e as any).tag === 'habit-counter')

        const decrypted = await Promise.all(
          counterEntries.map((e) =>
            decryptData<CounterPayload>(e.iv, e.ciphertext, encryptionKey).catch(() => null)
          )
        )

        set((state) => {
          const habits = state.habits.map((h) => {
            const relevantEntries = decrypted
              .filter((d): d is CounterPayload => d !== null && d.habitId === h.id)
            if (relevantEntries.length === 0) return h
            const existingDates = new Set(h.entries.map((e) => e.date))
            const newEntries = relevantEntries
              .filter((d) => !existingDates.has(d.date))
              .map((d) => ({ date: d.date, value: d.value }))
            return { ...h, entries: [...h.entries, ...newEntries] }
          })
          return { habits }
        })
      },

      reset: () => set(initialState),
    }),
    { name: 'habit-storage' }
  )
)
