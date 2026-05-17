import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { Todo } from '@/types'
import { todoService } from '@/services/todoService'
import { useUserStore } from './userStore'

// Normaliser les tâches anciennes ou mal formées pour assurer la cohérence des données
// Convertit les anciens objets timer en valeurs simples (minutes nullable)
function normalizeTodo(todo: Todo): Todo {
  const legacyTimer = todo.timer as
    | number
    | null
    | { allocated?: number; elapsed?: number; running?: boolean }

  if (legacyTimer && typeof legacyTimer === 'object') {
    const minutes = typeof legacyTimer.allocated === 'number' ? legacyTimer.allocated : null
    return {
      ...todo,
      timer: minutes,
      timerElapsed: legacyTimer.elapsed ?? 0,
      timerRunning: legacyTimer.running ?? false,
    }
  }

  return {
    ...todo,
    timer: legacyTimer ?? null,
    timerElapsed: todo.timerElapsed ?? 0,
    timerRunning: todo.timerRunning ?? false,
  }
}

interface TodoState {
  todos: Todo[]
  setTodos: (todos: Todo[]) => void
  addTodo: (todo: Todo) => Promise<void>
  updateTodo: (id: string, updates: Partial<Todo>) => Promise<void>
  deleteTodo: (id: string) => Promise<void>
  toggleComplete: (id: string) => Promise<void>
  reset: () => void
}

const initialState = { todos: [] }

export const useTodoStore = create<TodoState>()(
  persist(
    (set) => ({
      ...initialState,

      setTodos: (todos) => set({ todos }),

      addTodo: async (todo) => {
        const userId = useUserStore.getState().profile?.id
        if (!userId) return
        const created = await todoService.createTodo(userId, normalizeTodo(todo))
        set((state) => ({ todos: [...state.todos, normalizeTodo(created)] }))
      },

      updateTodo: async (id, updates) => {
        const userId = useUserStore.getState().profile?.id
        if (!userId) return
        const updated = await todoService.updateTodo(userId, id, updates)
        set((state) => ({ todos: state.todos.map((t) => (t.id === id ? normalizeTodo(updated) : normalizeTodo(t))) }))
      },

      deleteTodo: async (id) => {
        const userId = useUserStore.getState().profile?.id
        if (!userId) return
        await todoService.deleteTodo(userId, id)
        set((state) => ({ todos: state.todos.filter((t) => t.id !== id) }))
      },

      toggleComplete: async (id) => {
        const userId = useUserStore.getState().profile?.id
        if (!userId) return
        const current = useTodoStore.getState().todos.find((t) => t.id === id)
        if (!current) return
        const updated = await todoService.toggleComplete(userId, id, !current.completed)
        set((state) => ({ todos: state.todos.map((t) => (t.id === id ? normalizeTodo(updated) : normalizeTodo(t))) }))
      },

      reset: () => set(initialState),
    }),
    {
      name: 'todo-storage',
      version: 2,
      migrate: (persistedState: unknown) => {
        const state = persistedState as { todos?: Todo[] } | undefined

        return {
          ...(state ?? {}),
          todos: Array.isArray(state?.todos)
            ? state.todos.map((todo) => normalizeTodo(todo))
          : [],
        }
      },
    }
  )
)