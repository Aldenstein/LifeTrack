import { getJson, postJson } from './api'
import type { Todo, TodoPriority } from '@/types'
import type { ApiTodo, CreateTodoRequest } from '@/types/api'

const u = (userId: number) => `/users/${userId}/todos`

function priorityNumToStr(n?: number): TodoPriority {
  if (n === 3) return 'high'
  if (n === 1) return 'low'
  return 'medium'
}

function priorityStrToNum(p: TodoPriority | undefined): number | undefined {
  if (!p) return undefined
  return p === 'high' ? 3 : p === 'low' ? 1 : 2
}

function apiToApp(t: ApiTodo): Todo {
  return {
    id: String(t.id),
    title: t.title,
    description: (t as any).description ?? undefined,
    priority: priorityNumToStr((t as any).priority),
    tag: (t as any).tag ?? undefined,
    dueDate: (t as any).due_date ?? undefined,
    completed: (t as any).done ?? false,
    completedAt: (t as any).completed_at ?? undefined,
    timer: null,
    timerElapsed: 0,
    timerRunning: false,
    createdAt: (t as any).created_at ?? new Date().toISOString(),
  }
}

function appToApi(body: Todo): CreateTodoRequest {
  return {
    title: body.title,
    priority: priorityStrToNum(body.priority),
    due_date: body.dueDate,
  }
}

export const todoService = {
  // GET /users/{user_id}/todos — route absente, retourne [] sans erreur pour préserver l'état local
  getTodos: async (userId: number): Promise<Todo[]> => {
    try {
      const res = await getJson<ApiTodo[]>(u(userId))
      return res.map(apiToApp)
    } catch {
      return []
    }
  },

  // POST /users/{user_id}/todos — route présente ✅
  createTodo: async (userId: number, body: Todo) => {
    const payload = appToApi(body)
    const created = await postJson<ApiTodo>(u(userId), payload)
    return apiToApp(created)
  },

  // PUT /users/{user_id}/todos/{id} — route absente, mise à jour locale uniquement
  updateTodo: async (_userId: number, todoId: string, body: Partial<Todo>): Promise<Todo> => {
    return {
      id: todoId,
      title: body.title ?? '',
      priority: body.priority ?? 'medium',
      completed: body.completed ?? false,
      timer: body.timer ?? null,
      timerElapsed: body.timerElapsed ?? 0,
      timerRunning: body.timerRunning ?? false,
      createdAt: body.createdAt ?? new Date().toISOString(),
      ...body,
    } as Todo
  },

  // DELETE /users/{user_id}/todos/{id} — route absente, suppression locale uniquement
  deleteTodo: (_userId: number, _todoId: string): Promise<void> =>
    Promise.resolve(),

  // PATCH /users/{user_id}/todos/{id}/complete — route absente, toggle local uniquement
  toggleComplete: async (_userId: number, todoId: string, completed: boolean): Promise<Todo> => {
    return {
      id: todoId,
      title: '',
      priority: 'medium',
      completed,
      timer: null,
      timerElapsed: 0,
      timerRunning: false,
      createdAt: new Date().toISOString(),
    }
  },
}
