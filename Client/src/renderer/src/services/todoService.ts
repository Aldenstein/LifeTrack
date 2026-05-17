import { delJson, getJson, postJson, putJson, patchJson } from './api'
import type { Todo } from '@/types'

const u = (userId: number) => `/users/${userId}/todos`

export const todoService = {
  getTodos: (userId: number) => getJson<Todo[]>(u(userId)),
  createTodo: (userId: number, body: Todo) => postJson<Todo>(u(userId), body),
  updateTodo: (userId: number, todoId: string, body: Partial<Todo>) =>
    putJson<Todo>(`${u(userId)}/${todoId}`, body),
  deleteTodo: (userId: number, todoId: string) =>
    delJson<void>(`${u(userId)}/${todoId}`),
  toggleComplete: (userId: number, todoId: string, completed: boolean) =>
    patchJson<Todo>(`${u(userId)}/${todoId}/complete`, { completed }),
}