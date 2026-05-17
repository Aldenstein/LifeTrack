import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import type { User } from '@/types'

interface AuthState {
  user: User | null
  isAuthenticated: boolean
  login: (user: User) => void
  logout: () => void
}

export const useAuthStore = create<AuthState>()(
  persist(
    (set) => ({
      // État initial : utilisateur non identifié
      user: null,
      isAuthenticated: false,

      // Mettre à jour l'état lors de la connexion
      login: (user) => set({ user, isAuthenticated: true }),
      // Réinitialiser l'état lors de la déconnexion
      logout: () => set({ user: null, isAuthenticated: false }),
    }),
    { name: 'auth-storage' } // persisté dans localStorage pour session utilisateur
  )
)