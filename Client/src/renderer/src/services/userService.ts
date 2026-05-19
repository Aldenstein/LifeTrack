import { getJson, putJson } from './api'
import type { UpdateProfileRequest } from '@/types/api'

export const userService = {
  // GET /users/me/{user_id} → infos utilisateur courant
  // ✅ Correction : la route requiert l'user_id en paramètre (pas /users/me seul)
  getProfile: (userId: number) =>
    getJson<any>(`/users/me/${userId}`),

  // GET /users/{user_id}/profile → profil complet
  getFullProfile: (userId: number) =>
    getJson<any>(`/users/${userId}/profile`),

  // GET /users/{user_id}/dashboard/today → dashboard agrégé du jour
  getDashboardToday: (userId: number) =>
    getJson<any>(`/users/${userId}/dashboard/today`),

  // GET /users/{user_id}/latest-module-values → dernières valeurs de chaque module
  getLatestModuleValues: (userId: number) =>
    getJson<any>(`/users/${userId}/latest-module-values`),

  // GET /users/{user_id}/alerts-reminders → alertes et rappels actifs
  getAlertsReminders: (userId: number) =>
    getJson<any>(`/users/${userId}/alerts-reminders`),

  // PUT /users/me/{user_id} → mise à jour du profil
  updateProfile: (userId: number, body: UpdateProfileRequest) =>
    putJson<any>(`/users/${userId}/profile`, body),
}
