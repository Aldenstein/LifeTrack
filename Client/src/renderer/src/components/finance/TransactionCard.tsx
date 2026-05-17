import type { ApiTransaction, ApiFinanceType } from '@/types/api'

interface Props {
  transaction:  ApiTransaction
  financeTypes: ApiFinanceType[]
}

export default function TransactionCard({ transaction, financeTypes }: Props) {
  const type      = financeTypes.find(t => t.id === transaction.type_id)
  const isPositif = transaction.amount >= 0
  const date      = new Date(transaction.date).toLocaleDateString('fr-FR', {
    day: '2-digit', month: 'short', year: 'numeric',
  })

  return (
    <div className="mvt-item">
      <span className={`mvt-item__dot ${isPositif ? 'positif' : 'negatif'}`} />
      <div className="mvt-item__info">
        <span className="mvt-item__type">{type?.name ?? 'Sans type'}</span>
        <span className="mvt-item__date">
          {transaction.description && (
            <span className="mvt-item__desc">{transaction.description} · </span>
          )}
          {date}
        </span>
      </div>
      <div className="mvt-item__right">
        <span className={`mvt-item__montant ${isPositif ? 'positif' : 'negatif'}`}>
          {isPositif ? '+' : ''}
          {transaction.amount.toLocaleString('fr-FR', { style: 'currency', currency: 'EUR' })}
        </span>
      </div>
    </div>
  )
}
