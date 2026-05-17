import type { TodoPriority } from '@/types'

// Mapping : niveau de priorité -> label lisible + couleur UI
export const PRIORITY_CONFIG: Record<TodoPriority, { label: string; color: string }> = {
  high:   { label: 'Haute',   color: '#f0c070' },
  medium: { label: 'Moyenne', color: '#7ec8a4' },
  low:    { label: 'Basse',   color: '#8ab4d4' },
}