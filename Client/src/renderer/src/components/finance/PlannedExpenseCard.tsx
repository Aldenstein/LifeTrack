import type { ApiPlannedExpense, ApiFinanceType } from '@/types/api'

interface Props {
  expense:      ApiPlannedExpense
  financeTypes: ApiFinanceType[]
}

// Correction : helpers getJoursRestants et labelPeriode retires de ce composant
// Ils etaient dupliques depuis facture.ts avec des types differents (Facture vs ApiPlannedExpense)
// On les recalcule directement ici depuis next_date / periodicite / intervalle de l'API

function joursRestants(nextDate: string): number {
  const today = new Date(); today.setHours(0, 0, 0, 0)
  return Math.ceil((new Date(nextDate).getTime() - today.getTime()) / 86400000)
}

function labelPeriode(periodicite: string, intervalle: number): string {
  const map: Record<string, string> = { JOUR: 'jour', SEMAINE: 'semaine', MOIS: 'mois', ANNEE: 'an' }
  const unite = map[periodicite] ?? periodicite.toLowerCase()
  return intervalle === 1 ? `Tous les ${unite}s` : `Tous les ${intervalle} ${unite}s`
}

export default function PlannedExpenseCard({ expense, financeTypes }: Props) {
  const type      = financeTypes.find(t => t.id === expense.type_id)
  const joursRest = joursRestants(expense.next_date)
  const urgent    = joursRest <= 3
  const prochaine = new Date(expense.next_date).toLocaleDateString('fr-FR', {
    day: '2-digit', month: 'short',
  })

  return (
    <div className={`fac-item ${urgent ? 'fac-item--urgent' : ''}`}>
      <div className="fac-item__info">
        <span className="fac-item__type">{expense.description}</span>
        <span className="fac-item__periode">
          {type?.name && <>{type.name} · </>}
          {labelPeriode(expense.periodicite, expense.intervalle)}
        </span>
      </div>
      <div className="fac-item__date">
        <span className="fac-item__date-label">Prochain</span>
        <span className={`tag fac-item__tag ${urgent ? 'is-warning' : ''}`}>
          {urgent ? `J-${joursRest}` : prochaine}
        </span>
      </div>
      <div className="fac-item__right">
        <span className="fac-item__montant negatif">
          {expense.amount.toLocaleString('fr-FR', { style: 'currency', currency: 'EUR' })}
        </span>
      </div>
    </div>
  )
}
