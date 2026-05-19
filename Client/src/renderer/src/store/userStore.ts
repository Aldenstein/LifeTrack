import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { ApiProfile } from '@/types/api'

interface UserState {
  token: string | null
  profile: ApiProfile | null
  // ✅ Ajout : stockage des clés ZeroKnowledge reçues à la connexion/inscription
  encryptionKey: string | null
  encryptionSalt: string | null

  setToken: (token: string) => void
  setProfile: (profile: ApiProfile) => void
  // ✅ Ajout : setter pour les clés de chiffrement AES-256
  setEncryption: (key: string, salt: string) => void
  reset: () => void
}

const initialState = {
  token: null,
  profile: null,
  encryptionKey: null,
  encryptionSalt: null,
}

export const useUserStore = create<UserState>()(
  persist(
    (set) => ({
      ...initialState,

      setToken: (token) => set({ token }),
      setProfile: (profile) => set({ profile }),

      // Appelé après login/register pour persister les clés ZeroKnowledge
      // ⚠️ Si perdues, les données chiffrées ne peuvent plus être déchiffrées
      setEncryption: (encryptionKey, encryptionSalt) =>
        set({ encryptionKey, encryptionSalt }),

      reset: () => set(initialState),
    }),
    { name: 'user-storage' }
  )
)
