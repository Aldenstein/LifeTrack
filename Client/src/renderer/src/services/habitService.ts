import { delJson, getJson, postJson, putJson } from './api'
import type { Habit, HabitEntry } from '@/types'

const u = (userId: number) => `/users/${userId}/habits`

export const habitService = {
  getHabits: (userId: number) => getJson<Habit[]>(u(userId)),
  createHabit: (userId: number, body: Habit) => postJson<Habit>(u(userId), body),
  updateHabit: (userId: number, habitId: string, body: Partial<Habit>) =>
    putJson<Habit>(`${u(userId)}/${habitId}`, body),
  deleteHabit: (userId: number, habitId: string) =>
    delJson<void>(`${u(userId)}/${habitId}`),
  logEntry: (userId: number, habitId: string, entry: HabitEntry) =>
    postJson<Habit>(`${u(userId)}/${habitId}/entries`, entry),
}