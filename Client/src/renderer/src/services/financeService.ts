import { getJson, postJson } from './api'
import type {
  ApiAccount,
  ApiFinanceType,
  ApiTransaction,
  ApiPlannedExpense,
  CreateAccountRequest,
  CreateFinanceTypeRequest,
  CreateTransactionRequest,
  CreatePlannedExpenseRequest,
} from '@/types/api'

const u = (userId: number) => `/users/${userId}`

// Plage large pour récupérer toutes les transactions via l'endpoint /period
const ALL_START = '2000-01-01'
const ALL_END   = '2099-12-31'

export const financeService = {
  // GET /users/{user_id}/accounts
  getAccounts: (userId: number) =>
    getJson<ApiAccount[]>(`${u(userId)}/accounts`),

  // GET /finance/types
  getFinanceTypes: () =>
    getJson<ApiFinanceType[]>('/finance/types'),

  // GET /users/{user_id}/transactions/period — l'endpoint bare GET n'existe pas dans l'API
  getTransactions: (userId: number) =>
    getJson<ApiTransaction[]>(
      `${u(userId)}/transactions/period?start=${ALL_START}&end=${ALL_END}`
    ),

  // GET /users/{user_id}/planned-expenses
  getPlannedExpenses: (userId: number) =>
    getJson<ApiPlannedExpense[]>(`${u(userId)}/planned-expenses`),

  // POST /users/{user_id}/accounts
  createAccount: (userId: number, body: CreateAccountRequest) =>
    postJson<ApiAccount>(`${u(userId)}/accounts`, body),

  // POST /finance/types
  createFinanceType: (body: CreateFinanceTypeRequest) =>
    postJson<ApiFinanceType>('/finance/types', body),

  // POST /users/{user_id}/transactions
  createTransaction: (userId: number, body: CreateTransactionRequest) =>
    postJson<ApiTransaction>(`${u(userId)}/transactions`, body),

  // POST /users/{user_id}/planned-expenses
  createPlannedExpense: (userId: number, body: CreatePlannedExpenseRequest) =>
    postJson<ApiPlannedExpense>(`${u(userId)}/planned-expenses`, body),

  // ── Analytics ─────────────────────────────────────────────────────────────

  getAccountBalances: (userId: number) =>
    getJson<any>(`${u(userId)}/account-balances`),

  getTransactionsByPeriod: (userId: number, start: string, end: string) =>
    getJson<ApiTransaction[]>(`${u(userId)}/transactions/period?start=${start}&end=${end}`),

  getIncomePeriod: (userId: number, start: string, end: string) =>
    getJson<any>(`${u(userId)}/income/period?start=${start}&end=${end}`),

  getExpensesPeriod: (userId: number, start: string, end: string) =>
    getJson<any>(`${u(userId)}/expenses/period?start=${start}&end=${end}`),

  getBalancePeriod: (userId: number, start: string, end: string) =>
    getJson<any>(`${u(userId)}/balance/period?start=${start}&end=${end}`),

  getTopExpenses: (userId: number) =>
    getJson<any>(`${u(userId)}/top-expenses`),

  getBalanceHistory: (userId: number) =>
    getJson<any>(`${u(userId)}/balance-history`),
}
