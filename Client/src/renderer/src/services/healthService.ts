import { getJson, postJson } from './api'
import type { HydratationEntry, MoodEntry, MoodLevel, SleepEntry, SleepQuality, MealEntry, SportSessionEntry } from '@/types'

const u = (userId: number) => `/users/${userId}`

// ── Mappers API → types UI ─────────────────────────────────────────────────

function apiSleepToEntry(s: any): SleepEntry {
  return {
    id: String(s.id),
    date: s.date,
    time: s.time ?? '23:00',
    duration: s.duration ?? 0,
    quality: (Math.max(1, Math.min(5, Math.round(s.quality ?? 3))) as SleepQuality),
    is_restful: s.is_restful ?? false,
  }
}

function apiMealToEntry(m: any): MealEntry {
  return {
    id: String(m.id),
    date: m.date,
    time: m.time ?? '12:00:00',
    name: m.name,
    calories: m.calories ?? 0,
    proteins: m.proteins ?? 0,
    carbs: m.carbs ?? 0,
    fats: m.fats ?? 0,
  }
}

function apiSportToEntry(s: any): SportSessionEntry {
  return {
    id: String(s.id),
    date: s.date,
    time: s.time ?? '00:00:00',
    type_id: s.type_id ?? 1,
    duration: s.duration ?? 0,
    calories: s.calories ?? 0,
    intensity: s.intensity ?? 'modéré',
  }
}

// ── Service ────────────────────────────────────────────────────────────────

export const healthService = {

  // ── Hydratation ──────────────────────────────────────────────────────────

  getTodayHydration: (userId: number) =>
    getJson<any>(`${u(userId)}/hydration/today-total`),

  getHydrationGoalProgress: (userId: number) =>
    getJson<any>(`${u(userId)}/hydration/goal-progress`),

  getHydrationWeeklyAverage: (userId: number) =>
    getJson<any>(`${u(userId)}/hydration/average-weekly`),

  // Retourne les entrées agrégées par jour depuis l'historique
  getHydratationEntries: async (userId: number): Promise<HydratationEntry[]> => {
    try {
      const data = await getJson<any[]>(`${u(userId)}/hydration/history`)
      // Agréger par date (somme des quantités)
      const byDate: Record<string, HydratationEntry> = {}
      for (const entry of data) {
        const date = entry.date as string
        if (!byDate[date]) {
          byDate[date] = { date, quantity: 0, hydration_type: entry.hydration_type ?? 'eau', objective: entry.objective ?? 2000 }
        }
        byDate[date].quantity += entry.quantity ?? 0
      }
      return Object.values(byDate)
    } catch {
      return []
    }
  },

  logHydration: (userId: number, entry: {
    date: string; quantity: number; hydration_type: string; objective: number
  }) => postJson<any>(`${u(userId)}/hydration`, entry),

  // Alias utilisé par HydratationPage (poste l'incrément)
  upsertHydratationEntry: (userId: number, entry: HydratationEntry & { increment?: number }) =>
    postJson<any>(`${u(userId)}/hydration`, {
      date: entry.date,
      quantity: (entry as any).increment ?? entry.quantity,
      hydration_type: entry.hydration_type || 'eau',
      objective: entry.objective,
    }),

  // Pas de route DELETE dans l'API — no-op
  deleteHydratationEntry: (_userId: number, _date: string): Promise<void> =>
    Promise.resolve(),

  // ── Humeur ───────────────────────────────────────────────────────────────

  getMoodTypes: () =>
    getJson<any>('/mood/types'),

  getMoodToday: (userId: number) =>
    getJson<any>(`${u(userId)}/moods/today`),

  getMoodMostFrequent: (userId: number) =>
    getJson<any>(`${u(userId)}/moods/most-frequent`),

  getMoodDistribution: (userId: number) =>
    getJson<any>(`${u(userId)}/moods/distribution`),

  // Retourne les entrées d'humeur du mois courant
  getMoodEntries: async (userId: number): Promise<MoodEntry[]> => {
    try {
      const data = await getJson<any[]>(`${u(userId)}/moods/monthly`)
      return data.map((m) => ({
        id: String(m.id),
        date: m.date,
        mood_type_id: m.mood_type_id ?? 3,
        level: (Math.max(1, Math.min(5, m.mood_type_id ?? 3)) as MoodLevel),
      }))
    } catch {
      return []
    }
  },

  upsertMoodEntry: (userId: number, entry: MoodEntry) =>
    postJson<any>(`${u(userId)}/moods`, {
      date: entry.date,
      mood_type_id: entry.mood_type_id ?? entry.level ?? 3,
    }),

  // Pas de route DELETE dans l'API — no-op
  deleteMoodEntry: (_userId: number, _date: string): Promise<void> =>
    Promise.resolve(),

  logMood: (userId: number, entry: { date: string; mood_type_id: number }) =>
    postJson<any>(`${u(userId)}/moods`, entry),

  // ── Repas & Nutrition ────────────────────────────────────────────────────

  getCaloriesToday: (userId: number) =>
    getJson<any>(`${u(userId)}/nutrition/calories-today`),

  getMacroDistribution: (userId: number) =>
    getJson<any>(`${u(userId)}/nutrition/macro-distribution`),

  getNutritionHistory: (userId: number) =>
    getJson<any>(`${u(userId)}/nutrition/history`),

  // Retourne les repas du jour
  getMealEntries: async (userId: number): Promise<MealEntry[]> => {
    try {
      const data = await getJson<any[]>(`${u(userId)}/meals/today`)
      return data.map(apiMealToEntry)
    } catch {
      return []
    }
  },

  createMealEntry: async (userId: number, entry: MealEntry): Promise<MealEntry> => {
    const created = await postJson<any>(`${u(userId)}/meals`, {
      date: entry.date,
      time: entry.time || new Date().toTimeString().slice(0, 8),
      name: entry.name,
      calories: entry.calories,
      proteins: entry.proteins || 0,
      carbs: entry.carbs || 0,
      fats: entry.fats || 0,
    })
    return apiMealToEntry(created)
  },

  // Pas de route PUT dans l'API — retourne l'entrée telle quelle
  updateMealEntry: async (_userId: number, _id: string, entry: MealEntry): Promise<MealEntry> =>
    entry,

  // Pas de route DELETE dans l'API — no-op
  deleteMealEntry: (_userId: number, _id: string): Promise<void> =>
    Promise.resolve(),

  logMeal: (userId: number, entry: {
    date: string; time: string; name: string;
    calories: number; proteins: number; carbs: number; fats: number
  }) => postJson<any>(`${u(userId)}/meals`, entry),

  // ── Sport ────────────────────────────────────────────────────────────────

  getSportTypes: () =>
    getJson<any>('/sport/types'),

  getMostPracticedSport: (userId: number) =>
    getJson<any>(`${u(userId)}/sport/most-practiced`),

  getSportWeeklyStats: (userId: number) =>
    getJson<any>(`${u(userId)}/sport/stats-weekly`),

  // Retourne les séances du jour
  getSportSessions: async (userId: number): Promise<SportSessionEntry[]> => {
    try {
      const data = await getJson<any[]>(`${u(userId)}/sport-sessions/today`)
      return data.map(apiSportToEntry)
    } catch {
      return []
    }
  },

  createSportSession: async (userId: number, entry: SportSessionEntry): Promise<SportSessionEntry> => {
    const created = await postJson<any>(`${u(userId)}/sport-sessions`, {
      type_id: entry.type_id || 1,
      date: entry.date,
      time: entry.time || new Date().toTimeString().slice(0, 8),
      duration: entry.duration,
      calories: entry.calories,
      intensity: entry.intensity || 'modéré',
    })
    return apiSportToEntry(created)
  },

  // Pas de route PUT dans l'API — retourne l'entrée telle quelle
  updateSportSession: async (_userId: number, _id: string, entry: SportSessionEntry): Promise<SportSessionEntry> =>
    entry,

  // Pas de route DELETE dans l'API — no-op
  deleteSportSession: (_userId: number, _id: string): Promise<void> =>
    Promise.resolve(),

  logSportSession: (userId: number, entry: {
    type_id: number; date: string; time: string;
    duration: number; calories: number; intensity: string
  }) => postJson<any>(`${u(userId)}/sport-sessions`, entry),

  // ── Sommeil ──────────────────────────────────────────────────────────────

  getSleepToday: (userId: number) =>
    getJson<any>(`${u(userId)}/sleep/today`),

  getSleepWeeklyAverage: (userId: number) =>
    getJson<any>(`${u(userId)}/sleep/average-weekly`),

  getRestfulNights: (userId: number) =>
    getJson<any>(`${u(userId)}/sleep/restful`),

  // Retourne l'historique de sommeil
  getSleepEntries: async (userId: number): Promise<SleepEntry[]> => {
    try {
      const data = await getJson<any[]>(`${u(userId)}/sleep/history`)
      return data.map(apiSleepToEntry)
    } catch {
      return []
    }
  },

  upsertSleepEntry: async (userId: number, entry: SleepEntry): Promise<void> => {
    await postJson<any>(`${u(userId)}/sleep`, {
      date: entry.date,
      time: entry.time,
      duration: entry.duration,
      quality: entry.quality,
      is_restful: entry.is_restful,
    })
  },

  // Pas de route DELETE dans l'API — no-op
  deleteSleepEntry: (_userId: number, _date: string): Promise<void> =>
    Promise.resolve(),

  logSleep: (userId: number, entry: {
    date: string; time: string; duration: number; quality: number; is_restful: boolean
  }) => postJson<any>(`${u(userId)}/sleep`, entry),

  // ── Sobriété ─────────────────────────────────────────────────────────────

  getSobrietyCurrent: (userId: number) =>
    getJson<any>(`${u(userId)}/sobriety/current`),

  getSobrietyCurrentDuration: (userId: number) =>
    getJson<any>(`${u(userId)}/sobriety/current-duration`),

  getSobrietyHistory: (userId: number) =>
    getJson<any>(`${u(userId)}/sobriety/history`),

  getSobrietyTotalDuration: (userId: number) =>
    getJson<any>(`${u(userId)}/sobriety/total-duration`),

  getSobrietyStats: (userId: number) =>
    getJson<any>(`${u(userId)}/sobriety/stats`),

  startSobrietyPeriod: (userId: number, entry: {
    start_date: string; substance: string
  }) => postJson<any>(`${u(userId)}/sobriety-periods`, entry),

  // ── Mesures corporelles ──────────────────────────────────────────────────

  getWeightProgress: (userId: number) =>
    getJson<any>(`${u(userId)}/weight/progress`),

  getBMI: (userId: number) =>
    getJson<any>(`${u(userId)}/health/bmi-current`),

  getHealthMetrics: (userId: number) =>
    getJson<any>(`${u(userId)}/health/metrics`),

  logBodyMeasurement: (userId: number, entry: {
    date: string; weight: number; height: number;
    chest?: number; waist?: number; hips?: number
  }) => postJson<any>(`${u(userId)}/body-measurements`, entry),

  // ── Alcool ───────────────────────────────────────────────────────────────

  getBloodAlcoholLevel: (userId: number) =>
    getJson<any>(`${u(userId)}/alcohol/blood-alcohol-level`),

  logAlcoholConsumption: (userId: number, entry: {
    date: string; drink_type: string; quantity_ml: number; alcohol_percentage: number
  }) => postJson<any>(`${u(userId)}/alcohol-consumptions`, entry),

  // ── Respiration ──────────────────────────────────────────────────────────

  logBreathingSession: (userId: number, entry: {
    date: string; duration: number; technique: string
  }) => postJson<any>(`${u(userId)}/breathing-sessions`, entry),
}
