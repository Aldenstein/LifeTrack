export type FinancePeriod = 'JOUR' | 'SEMAINE' | 'MOIS' | 'ANNEE'

export interface TypeFinance {
  Typid:    string
  Typtitre: string
}

export interface Compte {
  Comid:    string
  Comnom:   string
  Comsolde: number
}

export interface Mouvement {
  Mouid:      string
  Moumontant: number
  Moudate:    string
  Typid:      string | null
  Comid:      string | null
}

export interface Facture {
  Facid:          string
  Facdate:        string
  Facperiodicite: FinancePeriod
  Facintervalle:  number
  Mouid:          string        // FK vers mouvement de référence
  Typid:          string | null
  Comid:          string | null
}