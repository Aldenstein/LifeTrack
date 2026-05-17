import { delJson, getJson, postJson, putJson } from './api'
import type {
  HydratationEntry,
  MoodEntry,
  MealEntry,
  SportSessionEntry,
  SleepEntry,
  SobrieteCard,
} from '@/types/health'

const base = (userId: number) => `/users/${userId}/health`

export const healthService = {
  getHydratationEntries: (userId: number) => getJson<HydratationEntry[]>(`${base(userId)}/hydratation`),
  upsertHydratationEntry: (userId: number, entry: HydratationEntry) =>
    putJson<HydratationEntry>(`${base(userId)}/hydratation/${entry.date}`, entry),

  getMoodEntries: (userId: number) => getJson<MoodEntry[]>(`${base(userId)}/mood`),
  upsertMoodEntry: (userId: number, entry: MoodEntry) =>
    putJson<MoodEntry>(`${base(userId)}/mood/${entry.date}`, entry),

  getMealEntries: (userId: number) => getJson<MealEntry[]>(`${base(userId)}/meals`),
  createMealEntry: (userId: number, entry: MealEntry) =>
    postJson<MealEntry>(`${base(userId)}/meals`, entry),
  deleteMealEntry: (userId: number, mealId: string) =>
    delJson<void>(`${base(userId)}/meals/${mealId}`),

  getSportSessions: (userId: number) => getJson<SportSessionEntry[]>(`${base(userId)}/sport`),
  createSportSession: (userId: number, entry: SportSessionEntry) =>
    postJson<SportSessionEntry>(`${base(userId)}/sport`, entry),
  deleteSportSession: (userId: number, sessionId: string) =>
    delJson<void>(`${base(userId)}/sport/${sessionId}`),

  getSleepEntries: (userId: number) => getJson<SleepEntry[]>(`${base(userId)}/sleep`),
  upsertSleepEntry: (userId: number, entry: SleepEntry) =>
    putJson<SleepEntry>(`${base(userId)}/sleep/${entry.date}`, entry),

  getSobrieteCards: (userId: number) => getJson<SobrieteCard[]>(`${base(userId)}/sobriety`),
  createSobrieteCard: (userId: number, card: SobrieteCard) =>
    postJson<SobrieteCard>(`${base(userId)}/sobriety`, card),
  updateSobrieteCard: (userId: number, cardId: string, card: Partial<SobrieteCard>) =>
    putJson<SobrieteCard>(`${base(userId)}/sobriety/${cardId}`, card),
  deleteSobrieteCard: (userId: number, cardId: string) =>
    delJson<void>(`${base(userId)}/sobriety/${cardId}`),
}