import { getJson, putJson } from './api'
import type { ApiProfile, UpdateProfileRequest } from '@/types/api'

export const userService = {
  getProfile:    ()                       => getJson<ApiProfile>(  '/users/me'),
  updateProfile: (body: UpdateProfileRequest) => putJson<ApiProfile>('/users/me', body),
}
