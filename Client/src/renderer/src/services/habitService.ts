import { getJson, postJson } from './api'
import type { Habit } from '@/types'
import type { ApiHabit, CreateHabitRequest } from '@/types/api'

const u = (userId: number) => `/users/${userId}/habits`

function apiToApp(h: ApiHabit): Habit {
  return {
    id: String(h.id),
    name: h.title,
    icon: '🏷️',
    color: '#888',
    type: 'check',
    // L'API retourne 'DAILY' | 'CUSTOM' en majuscules
    frequency: (h.frequency === 'DAILY' ? 'daily' : 'custom') as any,
    customDays: undefined,
    unit: undefined,
    goal: undefined,
    step: undefined,
    entries: [],
    streak: 0,
    reward: 1,
    createdAt: h.created_at,
  }
}

function appToApi(body: Habit): CreateHabitRequest {
  return {
    title: body.name,
    description: '',
    frequency: body.frequency === 'daily' ? 'DAILY' : 'CUSTOM',
  }
}

export const habitService = {
  // GET /users/{user_id}/habits/active
  getHabitsActive: async (userId: number) => {
    const res = await getJson<ApiHabit[]>(`${u(userId)}/active`)
    return res.map(apiToApp)
  },

  // GET /users/{user_id}/habits/today
  getHabitsToday: async (userId: number) => {
    const res = await getJson<ApiHabit[]>(`${u(userId)}/today`)
    return res.map(apiToApp)
  },

  getScoreToday: (userId: number) =>
    getJson<any>(`${u(userId)}/score-today`),

  getScoreWeekly: (userId: number) =>
    getJson<any>(`${u(userId)}/score-weekly`),

  getMostConsistent: (userId: number) =>
    getJson<any>(`${u(userId)}/most-consistent`),

  getCompletionRate: (userId: number, habitId: string) =>
    getJson<any>(`${u(userId)}/${habitId}/completion-rate`),

  // POST /users/{user_id}/habits
  createHabit: async (userId: number, body: Habit) => {
    const payload = appToApi(body)
    const created = await postJson<ApiHabit>(`${u(userId)}`, payload)
    return apiToApp(created)
  },

  // PUT /users/{user_id}/habits/{habit_id} — route absente de l'API, mise à jour locale uniquement
  updateHabit: async (_userId: number, habitId: string, body: Partial<Habit>): Promise<Habit> => {
    return {
      id: habitId,
      name: body.name ?? '',
      icon: body.icon ?? '🏷️',
      color: body.color ?? '#888',
      type: body.type ?? 'check',
      frequency: body.frequency ?? 'daily',
      entries: body.entries ?? [],
      streak: body.streak ?? 0,
      reward: body.reward ?? 1,
      createdAt: body.createdAt ?? new Date().toISOString(),
      ...body,
    } as Habit
  },

  // DELETE /users/{user_id}/habits/{habit_id} — route absente de l'API, suppression locale uniquement
  deleteHabit: (_userId: number, _habitId: string): Promise<void> =>
    Promise.resolve(),

  // POST /users/{user_id}/habits/{habit_id}/complete
  completeHabit: (userId: number, habitId: string, date: string) =>
    postJson<any>(`${u(userId)}/${habitId}/complete`, { date }),
}
