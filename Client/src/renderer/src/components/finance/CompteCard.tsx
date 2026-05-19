import type { ApiAccount, ApiTransaction } from '@/types/api'

interface Props {
  account:      ApiAccount
  transactions: ApiTransaction[]
  onClick:      (id: number) => void
  onDelete:     (id: number) => void
}

export default function CompteCard({ account, transactions, onClick, onDelete }: Props) {
  const txCount = transactions.filter(t => t.account_id === account.id).length

  return (
    <div className="box compte-card" onClick={() => onClick(account.id)}>
      <div className="media compte-card__main">
        <div className="media-left">
          <div className="compte-card__icon">💶</div>
        </div>
        <div className="media-content">
          <p className="compte-card__nom">{account.name}</p>
          <p className="compte-card__meta">{txCount} mouvement{txCount !== 1 ? 's' : ''}</p>
        </div>
        <div className="media-right">
          <div className="compte-card__balance">
            <p className={`title compte-card__solde ${account.balance >= 0 ? 'positif' : 'negatif'}`}>
              {account.balance.toLocaleString('fr-FR', { style: 'currency', currency: 'EUR' })}
            </p>
          </div>
          <div className="compte-card__actions">
            <div className="buttons are-small">
              <button
                className="button compte-card__btn"
                onClick={e => { e.stopPropagation(); onClick(account.id) }}
                title="Voir">✏️</button>
              {/* Correction : onDelete branchée — avant elle etait () => {} dans FinancePage */}
              <button
                className="button compte-card__btn compte-card__btn--danger"
                onClick={e => { e.stopPropagation(); onDelete(account.id) }}
                title="Supprimer">🗑️</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}
