import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { Todo } from '@/types'
import { todoService } from '@/services/todoService'
import { useUserStore } from './userStore'

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
    (set, get) => ({
      ...initialState,

      setTodos: (todos) => set({ todos }),

      addTodo: async (todo) => {
        const userId = useUserStore.getState().profile?.id
        if (!userId) return
        try {
          const created = await todoService.createTodo(userId, normalizeTodo(todo))
          set((state) => ({ todos: [...state.todos, normalizeTodo(created)] }))
        } catch {
          // Si l'API échoue, on insère quand même localement
          set((state) => ({ todos: [...state.todos, normalizeTodo(todo)] }))
        }
      },

      // Pas de route PUT dans l'API — mise à jour locale uniquement
      updateTodo: async (id, updates) => {
        set((state) => ({
          todos: state.todos.map((t) =>
            t.id === id ? normalizeTodo({ ...t, ...updates }) : normalizeTodo(t)
          ),
        }))
      },

      // Pas de route DELETE dans l'API — suppression locale uniquement
      deleteTodo: async (id) => {
        set((state) => ({ todos: state.todos.filter((t) => t.id !== id) }))
      },

      // Pas de route PATCH dans l'API — toggle local uniquement
      toggleComplete: async (id) => {
        const current = get().todos.find((t) => t.id === id)
        if (!current) return
        set((state) => ({
          todos: state.todos.map((t) =>
            t.id === id ? normalizeTodo({ ...t, completed: !t.completed }) : normalizeTodo(t)
          ),
        }))
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
