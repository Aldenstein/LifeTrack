// ─────────── Authentification ───────────
export interface LoginRequest  { email: string; password: string }
export interface RegisterRequest { email: string; password: string; username: string }
export interface AuthResponse  { token: string; user_id: number }

// ─────────── Utilisateur ───────────
export interface ApiProfile {
  id:         number
  username:   string
  email:      string
  created_at: string
  avatar_url?: string
}

export interface UpdateProfileRequest {
  username?:   string
  email?:      string
  avatar_url?: string
}

// ─────────── Finance ───────────
export interface ApiAccount {
  id:      number
  name:    string
  balance: number
}

export interface ApiFinanceType {
  id:   number
  name: string
}

export interface ApiTransaction {
  id:          number
  account_id:  number
  type_id:     number
  amount:      number
  description: string
  date:        string
}

export interface ApiPlannedExpense {
  id:          number
  account_id:  number
  type_id:     number
  amount:      number
  description: string
  periodicite: string
  intervalle:  number
  next_date:   string
}

// ─────────── Tâches ───────────
export interface ApiTodo {
  id:          number
  title:       string
  done:        boolean
  priority:    number
  due_date?:   string
  created_at:  string
}

// ─────────── Habitudes ───────────
export interface ApiHabit {
  id:          number
  title:       string
  description: string
  frequency:   string
  created_at:  string
}

export interface ApiHabitEntry {
  id:       number
  habit_id: number
  date:     string
  done:     boolean
}

// ─────────── Commun ───────────
export interface CreatedResponse { id: number }

// ─────────── Corps des requêtes ───────────
export interface CreateAccountRequest      { name: string; balance: number }
export interface CreateFinanceTypeRequest  { name: string }
export interface CreateTransactionRequest  { account_id: number; type_id: number; amount: number; description: string }
export interface CreatePlannedExpenseRequest {
  account_id:  number
  type_id:     number
  amount:      number
  description: string
  periodicite: string
  intervalle:  number
  next_date:   string
}
export interface CreateTodoRequest        { title: string; priority?: number; due_date?: string }
export interface CreateHabitRequest       { title: string; description: string; frequency: string }
export interface CreateHabitEntryRequest  { habit_id: number; date: string; done: boolean }
