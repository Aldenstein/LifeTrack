// Niveaux de priorité possibles pour une tâche
export type TodoPriority = 'high' | 'medium' | 'low'

// Schéma d'une tâche (modèle UI — mappé depuis ApiTodo par todoService)
export interface Todo {
  id: string
  title: string
  description?: string
  priority: TodoPriority
  tag?: string
  dueDate?: string       // ISO date — mappé depuis due_date
  completed: boolean     // mappé depuis done
  completedAt?: string   // mappé depuis completed_at
  timer: number | null   // minutes sélectionnées, null = aucun minuteur
  timerElapsed: number   // secondes écoulées (défaut 0)
  timerRunning: boolean  // minuteur actif (défaut false)
  createdAt: string      // mappé depuis created_at
}
