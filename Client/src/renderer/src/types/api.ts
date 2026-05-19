// ─────────── Authentification ───────────────────────────────────────────────
export interface LoginRequest {
  email: string
  // ✅ Correction : l'API utilise "passphrase" et non "password"
  passphrase: string
}

export interface RegisterRequest {
  email: string
  // ✅ Correction : l'API utilise "passphrase" et non "password"
  //    Le champ "username" n'existe pas dans l'API LifeTrack
  passphrase: string
}

export interface AuthResponse {
  token: string
  user_id: number
  email: string
  // ✅ Ajout : clés ZeroKnowledge reçues à chaque login/register
  //    À stocker immédiatement — si perdues, données chiffrées irrécupérables
  encryption_key: string  // clé AES-256 hex (32 bytes)
  encryption_salt: string // salt hex (16 bytes)
}

// Réponse de /auth/derive-keys (sans nouveau token JWT)
export interface DeriveKeysResponse {
  encryption_key: string
  salt: string
}

// ─────────── Utilisateur ────────────────────────────────────────────────────
export interface ApiProfile {
  id: number
  email: string
  created_at: string
  // ✅ username et avatar_url sont optionnels car absents de la doc API
  username?: string
  avatar_url?: string
}

export interface UpdateProfileRequest {
  username?: string
  email?: string
  avatar_url?: string
}

// ─────────── Finance ────────────────────────────────────────────────────────
export interface ApiAccount {
  id: number
  name: string
  balance: number
}

export interface ApiFinanceType {
  id: number
  name: string
}

export interface ApiTransaction {
  id: number
  account_id: number
  type_id: number
  amount: number
  description: string
  date: string
}

export interface ApiPlannedExpense {
  id: number
  account_id: number
  type_id: number
  amount: number
  description: string
  // ✅ Nommage aligné avec la doc API (periodicite / intervalle)
  periodicite: string
  intervalle: number
  next_date: string
}

// ─────────── Tâches ─────────────────────────────────────────────────────────
export interface ApiTodo {
  id: number
  title: string
  done: boolean
  priority: number
  due_date?: string
  created_at: string
}

// ─────────── Habitudes ──────────────────────────────────────────────────────
export interface ApiHabit {
  id: number
  title: string
  description: string
  // ✅ L'API retourne 'DAILY' | 'CUSTOM' en majuscules
  frequency: 'DAILY' | 'CUSTOM' | string
  created_at: string
}

// ─────────── Santé — Réponses API ───────────────────────────────────────────
export interface ApiHydration {
  date: string
  quantity: number
  hydration_type: string
  objective: number
}

export interface ApiMood {
  id: number
  date: string
  mood_type_id: number
}

export interface ApiMoodType {
  id: number
  name: string
  emoji?: string
}

export interface ApiMeal {
  id: number
  date: string
  time: string
  name: string
  calories: number
  proteins: number
  carbs: number
  fats: number
}

export interface ApiSportSession {
  id: number
  type_id: number
  date: string
  time: string
  duration: number
  calories: number
  intensity: string
}

export interface ApiSportType {
  id: number
  name: string
}

export interface ApiSleep {
  id: number
  date: string
  time: string
  duration: number  // minutes
  quality: number   // 0.0 – 5.0
  is_restful: boolean
}

export interface ApiSobrietyPeriod {
  id: number
  start_date: string
  substance: string
  end_date?: string
}

export interface ApiBodyMeasurement {
  id: number
  date: string
  weight: number
  height: number
  chest?: number
  waist?: number
  hips?: number
}

export interface ApiAlcoholConsumption {
  id: number
  date: string
  drink_type: string
  quantity_ml: number
  alcohol_percentage: number
}

// ─────────── Commun ──────────────────────────────────────────────────────────
export interface CreatedResponse { id: number }

// ─────────── Corps des requêtes ──────────────────────────────────────────────
export interface CreateAccountRequest { name: string; balance: number }
export interface CreateFinanceTypeRequest { name: string }
export interface CreateTransactionRequest {
  account_id: number
  type_id: number
  amount: number
  description: string
  date?: string
}
export interface CreatePlannedExpenseRequest {
  account_id: number
  type_id: number
  amount: number
  description: string
  periodicite: string
  intervalle: number
  next_date: string
}
export interface CreateTodoRequest {
  title: string
  priority?: number
  due_date?: string
}
export interface CreateHabitRequest {
  title: string
  description: string
  // ✅ L'API attend 'DAILY' | 'CUSTOM' en majuscules
  frequency: 'DAILY' | 'CUSTOM'
}
export interface CreateMealRequest {
  date: string
  time: string
  name: string
  calories: number
  proteins: number
  carbs: number
  fats: number
}
export interface CreateSleepRequest {
  date: string
  time: string
  duration: number
  quality: number
  is_restful: boolean
}
export interface CreateSportSessionRequest {
  type_id: number
  date: string
  time: string
  duration: number
  calories: number
  intensity: string
}
export interface CreateHydrationRequest {
  date: string
  quantity: number
  hydration_type: string
  objective: number
}
export interface CreateBodyMeasurementRequest {
  date: string
  weight: number
  height: number
  chest?: number
  waist?: number
  hips?: number
}
export interface CreateSobrietyPeriodRequest {
  start_date: string
  substance: string
}
export interface CreateAlcoholConsumptionRequest {
  date: string
  drink_type: string
  quantity_ml: number
  alcohol_percentage: number
}
export interface CreateBreathingSessionRequest {
  date: string
  duration: number
  technique: string
}
