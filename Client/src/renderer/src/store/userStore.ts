import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { ApiProfile } from '@/types/api'

interface UserState {
  token:   string | null
  profile: ApiProfile | null

  setToken:   (token: string) => void
  setProfile: (profile: ApiProfile) => void
  reset:      () => void
}

export const useUserStore = create<UserState>()(
  persist(
    (set) => ({
      token:   null,
      profile: null,

      setToken:   (token)   => set({ token }),
      setProfile: (profile) => set({ profile }),
      reset:      ()        => set({ token: null, profile: null }),
    }),
    { name: 'user-storage' }
  )
)
