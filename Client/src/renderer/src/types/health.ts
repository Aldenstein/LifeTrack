// ============================================================
// types/health.ts — Types TypeScript pour le module Santé
// Chaque sous-module a ses propres interfaces ici
// ============================================================

// --- SOMMEIL ---
export type SleepQuality = 1 | 2 | 3 | 4 | 5;

export interface SleepEntry {
  id: string;
  date: string;        // ISO "2026-05-17"
  bedTime: string;     // "23:30"
  wakeTime: string;    // "07:15"
  quality: SleepQuality;
  note?: string;
}

// --- HYDRATATION ---
export interface HydratationEntry {
  date: string;        // ISO "2026-05-17"
  totalMl: number;     // total bu dans la journée
  goal: number;        // objectif en ml (défaut 2000)
}

// --- REPAS / CALORIES ---
export type MealType = 'breakfast' | 'lunch' | 'dinner' | 'snack';

export interface MealEntry {
  id: string;
  date: string;        // ISO "2026-05-17"
  mealType: MealType;
  name: string;
  calories: number;
  note?: string;
}

// --- SPORT / CALORIES BRÛLÉES ---
export type SportType = 'running' | 'cycling' | 'walking' | 'musculation' | 'hiit' | 'other';

export interface SportSessionEntry {
  id: string;
  date: string;        // ISO "2026-05-17"
  sportType: SportType;
  durationMin: number;
  weightKg: number;
  met: number;
  caloriesBurned: number;
  note?: string;
}

// --- HUMEUR ---
export type MoodLevel = 1 | 2 | 3 | 4 | 5;

export interface MoodEntry {
  id: string;
  date: string;        // ISO "2026-05-17"
  level: MoodLevel;    // 1 = très mauvaise → 5 = excellente
  note?: string;
}

// --- CALCULS PONDÉRAUX ---
export type Gender = 'male' | 'female';
export type ActivityLevel =
  | 'sedentary'    // ×1.2
  | 'light'        // ×1.375
  | 'moderate'     // ×1.55
  | 'active'       // ×1.725
  | 'very_active'; // ×1.9

// --- ALCOOLÉMIE ---
export type DrinkType = 'beer' | 'wine' | 'shot' | 'cocktail' | 'cider';

export interface DrinkItem {
  type: DrinkType;
  quantity: number;   // nombre de verres
}

// --- SOBRIÉTÉ ---
export interface SobrieteCard {
  id: string;
  substance: string;   // "Alcool", "Tabac", "Cannabis"...
  startDate: string;   // ISO "2026-05-17" — date de début de sobriété
  color: string;       // couleur hex de la carte ex: "#5cad7a"
  icon: string;        // emoji ex: "🚬"
}

// --- COHÉRENCE CARDIAQUE ---
export type BreathingProgramId =
  | 'coherence'    // 5/5 — cohérence cardiaque
  | 'energizing'   // 4/2/4 — énergisant
  | 'calming'      // 4/7/8 — calmant
  | 'box';         // 4/4/4/4 — box breathing

export interface BreathingProgram {
  id: BreathingProgramId;
  label: string;
  description: string;
  inhale: number;    // secondes
  holdIn?: number;   // secondes (optionnel)
  exhale: number;    // secondes
  holdOut?: number;  // secondes (optionnel)
  color: string;     // couleur accent du programme
}
