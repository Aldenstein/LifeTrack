// ============================================================
// types/health.ts — Types TypeScript pour le module Santé
// Modèles UI mappés depuis les réponses API (types/api.ts)
// ============================================================

// --- SOMMEIL ─────────────────────────────────────────────────
export type SleepQuality = 1 | 2 | 3 | 4 | 5

export interface SleepEntry {
  id: string
  date: string         // ISO "2026-05-17"
  time: string         // "23:30:00" — heure de coucher
  duration: number     // ✅ Correction : l'API stocke la durée en minutes (pas bedTime/wakeTime)
  quality: SleepQuality
  is_restful: boolean  // ✅ Ajout : champ présent dans l'API
  note?: string        // local uniquement, non stocké en base
}

// --- HYDRATATION ─────────────────────────────────────────────
export interface HydratationEntry {
  date: string          // ISO "2026-05-17"
  quantity: number      // ✅ Correction : quantité d'une entrée (pas total journalier)
  hydration_type: string // ex: "eau", "jus", "thé"
  objective: number     // objectif en ml (défaut 2000)
}

// --- REPAS / CALORIES ────────────────────────────────────────
// ✅ Correction : MealType local conservé pour l'UI (non présent dans l'API)
export type MealType = 'breakfast' | 'lunch' | 'dinner' | 'snack'

export interface MealEntry {
  id: string
  date: string         // ISO "2026-05-17"
  time: string         // "12:30:00"
  name: string
  calories: number
  // ✅ Ajout : macros présents dans le payload API
  proteins: number
  carbs: number
  fats: number
  mealType?: MealType  // local uniquement, non stocké en base
  note?: string
}

// --- SPORT ───────────────────────────────────────────────────
// ✅ Correction : SportType local conservé pour l'UI
//    L'API utilise type_id (FK vers /sport/types)
export type SportType = 'running' | 'cycling' | 'walking' | 'musculation' | 'hiit' | 'other'

export interface SportSessionEntry {
  id: string
  date: string          // ISO "2026-05-17"
  time: string          // "07:00:00"
  type_id: number       // ✅ Ajout : FK vers ApiSportType
  sportType?: SportType // local uniquement (affichage UI)
  duration: number      // ✅ Correction : durée en minutes (aligné API)
  calories: number
  intensity: string     // "léger" | "modéré" | "intense"
  note?: string
}

// --- HUMEUR ──────────────────────────────────────────────────
export type MoodLevel = 1 | 2 | 3 | 4 | 5

export interface MoodEntry {
  id: string
  date: string         // ISO "2026-05-17"
  mood_type_id: number // ✅ Correction : l'API utilise un type d'humeur par ID
  level?: MoodLevel    // local uniquement (calculé depuis mood_type)
  note?: string
}

// --- CALCULS PONDÉRAUX ───────────────────────────────────────
export type Gender = 'male' | 'female'
export type ActivityLevel =
  | 'sedentary'    // ×1.2
  | 'light'        // ×1.375
  | 'moderate'     // ×1.55
  | 'active'       // ×1.725
  | 'very_active'  // ×1.9

// --- MESURES CORPORELLES ─────────────────────────────────────
// ✅ Ajout : interface manquante pour /body-measurements
export interface BodyMeasurement {
  id: string
  date: string
  weight: number
  height: number
  chest?: number
  waist?: number
  hips?: number
}

// --- ALCOOLÉMIE ──────────────────────────────────────────────
export type DrinkType = 'beer' | 'wine' | 'shot' | 'cocktail' | 'cider'

export interface DrinkItem {
  type: DrinkType
  quantity: number // nombre de verres
}

// ✅ Ajout : interface pour les consommations stockées en base
export interface AlcoholConsumption {
  id: string
  date: string
  drink_type: string
  quantity_ml: number
  alcohol_percentage: number
}

// --- SOBRIÉTÉ ────────────────────────────────────────────────
export interface SobrieteCard {
  id: string
  substance: string   // "Alcool", "Tabac", "Cannabis"...
  // ✅ Correction : start_date (snake_case aligné avec l'API)
  start_date: string  // ISO "2026-05-17"
  end_date?: string   // présent si la période est terminée
  // Champs UI locaux — non stockés en base
  color: string       // couleur hex ex: "#5cad7a"
  icon: string        // emoji ex: "🚬"
}

// --- COHÉRENCE CARDIAQUE ─────────────────────────────────────
export type BreathingProgramId =
  | 'coherence'   // 5/5 — cohérence cardiaque
  | 'energizing'  // 4/2/4 — énergisant
  | 'calming'     // 4/7/8 — calmant
  | 'box'         // 4/4/4/4 — box breathing

export interface BreathingProgram {
  id: BreathingProgramId
  label: string
  description: string
  inhale: number    // secondes
  holdIn?: number   // secondes (optionnel)
  exhale: number    // secondes
  holdOut?: number  // secondes (optionnel)
  color: string     // couleur accent du programme
}

// ✅ Ajout : interface pour les séances stockées en base
export interface BreathingSession {
  id: string
  date: string
  duration: number   // secondes
  technique: string  // id du programme
}
