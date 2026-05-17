// Niveaux de priorité possibles pour une tâche
export type TodoPriority = 'high' | 'medium' | 'low'

// Schéma d'une tâche
export interface Todo {
  id: string
  title: string
  description?: string
  priority: TodoPriority
  tag?: string
  dueDate?: string      // ISO date
  completed: boolean
  completedAt?: string
  timer: number | null  // minutes sélectionnées, null = aucun minuteur
  timerElapsed?: number // secondes écoulées du minuteur
  timerRunning?: boolean
  createdAt: string
}