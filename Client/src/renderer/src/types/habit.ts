export type HabitType = 'check' | 'counter'
export type HabitFrequency = 'daily' | 'weekly' | 'custom'
export type HabitReward = -1 | 1

export interface HabitEntry {
  date: string   // format ISO "2026-04-06"
  value: number  // 1 = fait, ou valeur du compteur
}

// Modèle UI — mappé depuis ApiHabit par habitService
export interface Habit {
  id: string
  name: string       // mappé depuis title
  icon: string
  color: string
  type: HabitType
  frequency: HabitFrequency
  customDays?: number[]  // [1,3,5] = lun, mer, ven
  // Pour les compteurs uniquement
  unit?: string   // "ml", "reps", "min"
  goal?: number   // 2000, 50…
  step?: number   // incrément rapide, ex: 250
  // Suivi local (non stocké en base, calculé côté client)
  entries: HabitEntry[]
  streak: number
  reward: HabitReward
  createdAt: string
}
