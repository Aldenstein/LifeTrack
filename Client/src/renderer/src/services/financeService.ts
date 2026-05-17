import { getJson, postJson } from './api'
import type {
  ApiAccount,
  ApiFinanceType,
  ApiTransaction,
  ApiPlannedExpense,
  CreatedResponse,
  CreateAccountRequest,
  CreateFinanceTypeRequest,
  CreateTransactionRequest,
  CreatePlannedExpenseRequest,
} from '@/types/api'

const u = (userId: number) => `/users/${userId}`

export const financeService = {

  // ── GET ──────────────────────────────────────────
  getAccounts: (userId: number) =>
    getJson<ApiAccount[]>(`${u(userId)}/accounts`),

  getFinanceTypes: () =>
    getJson<ApiFinanceType[]>(`/finance/types`),

  getTransactions: (userId: number) =>
    getJson<ApiTransaction[]>(`${u(userId)}/transactions`),

  getPlannedExpenses: (userId: number) =>
    getJson<ApiPlannedExpense[]>(`${u(userId)}/planned-expenses`),

  // ── POST ─────────────────────────────────────────
  createAccount: (userId: number, body: CreateAccountRequest) =>
    postJson<CreatedResponse>(`${u(userId)}/accounts`, body),

  createFinanceType: (body: CreateFinanceTypeRequest) =>
    postJson<CreatedResponse>(`/finance/types`, body),

  createTransaction: (userId: number, body: CreateTransactionRequest) =>
    postJson<CreatedResponse>(`${u(userId)}/transactions`, body),

  createPlannedExpense: (userId: number, body: CreatePlannedExpenseRequest) =>
    postJson<CreatedResponse>(`${u(userId)}/planned-expenses`, body),
}