import type { Facture, FinancePeriod } from '@/types/finance'

function addPeriod(date: Date, intervalle: number, periodicite: FinancePeriod): Date {
  const d = new Date(date)
  switch (periodicite) {
    case 'JOUR':    d.setDate(d.getDate() + intervalle);         break
    case 'SEMAINE': d.setDate(d.getDate() + intervalle * 7);     break
    case 'MOIS':    d.setMonth(d.getMonth() + intervalle);       break
    case 'ANNEE':   d.setFullYear(d.getFullYear() + intervalle); break
  }
  return d
}

export function getProchainDate(facture: Facture): string {
  let date = new Date(facture.Facdate)
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  while (date <= today) {
    date = addPeriod(date, facture.Facintervalle, facture.Facperiodicite)
  }
  return date.toISOString().split('T')[0]
}

export function getJoursRestants(facture: Facture): number {
  const prochain = new Date(getProchainDate(facture))
  const today    = new Date()
  today.setHours(0, 0, 0, 0)
  return Math.ceil((prochain.getTime() - today.getTime()) / 86400000)
}

export function labelPeriode(facture: Facture): string {
  const map: Record<string, string> = {
    JOUR:    'jour', SEMAINE: 'semaine', MOIS: 'mois', ANNEE: 'an'
  }
  const unite = map[facture.Facperiodicite] ?? facture.Facperiodicite.toLowerCase()
  return facture.Facintervalle === 1
    ? `Tous les ${unite}s`
    : `Tous les ${facture.Facintervalle} ${unite}s`
}