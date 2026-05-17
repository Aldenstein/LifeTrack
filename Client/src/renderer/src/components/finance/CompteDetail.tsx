import { useState } from 'react'
import BackButton from '@/components/BackButton'
import { useFinanceStore }       from '@/store/useFinanceStore'
import TransactionCard           from './TransactionCard'
import TransactionModal          from './TransactionModal'
import PlannedExpenseCard        from './PlannedExpenseCard'
import PlannedExpenseModal       from './PlannedExpenseModal'

interface Props {
  accountId: number
  onBack:    () => void
}

type Tab = 'transactions' | 'planned'

export default function CompteDetail({ accountId, onBack }: Props) {
  const { accounts, transactions, plannedExpenses, financeTypes } = useFinanceStore()

  const account  = accounts.find(a => a.id === accountId)
  const txs      = transactions
    .filter(t => t.account_id === accountId)
    .sort((a, b) => b.date.localeCompare(a.date))
  const planned  = plannedExpenses.filter(e => e.account_id === accountId)

  const [tab,          setTab]          = useState<Tab>('transactions')
  const [showTxModal,  setShowTxModal]  = useState(false)
  const [showFacModal, setShowFacModal] = useState(false)

  if (!account) return null

  const entrees = txs.filter(t => t.amount > 0).reduce((s, t) => s + t.amount, 0)
  const sorties = txs.filter(t => t.amount < 0).reduce((s, t) => s + t.amount, 0)
  const fmt     = (n: number) => n.toLocaleString('fr-FR', { style: 'currency', currency: 'EUR' })

  return (
    <>
      <section className="hero is-fullheight finance-bg">
        <div className="hero-body is-align-items-flex-start" style={{ paddingTop: '2rem' }}>
          <div className="container">
            <div className="finance-page-header">
              <div className="level is-mobile mb-0">
                <div className="level-left">
                  <div className="level-item" style={{ gap: '.75rem' }}>
                    <BackButton label="←" onClick={onBack} />
                    <div>
                      <p className="finance-title">{account.name}</p>
                      <p className="finance-subtitle">
                        {tab === 'transactions' ? 'Mouvements' : 'Dépenses planifiées'}
                      </p>
                    </div>
                  </div>
                </div>
                <div className="level-right">
                  <div className="level-item">
                    <button className="button finance-add-btn"
                      onClick={() => tab === 'transactions' ? setShowTxModal(true) : setShowFacModal(true)}>
                      +
                    </button>
                  </div>
                </div>
              </div>
            </div>

            <div className="box finance-box finance-box--wide">

              <div className="columns is-mobile is-gapless compte-stats">
                <div className="column has-text-centered">
                  <p className="heading finance-solde-total__label">Solde</p>
                  <p className={`title compte-stats__value ${account.balance >= 0 ? 'positif' : 'negatif'}`}>
                    {fmt(account.balance)}
                  </p>
                </div>
                <div className="column has-text-centered">
                  <p className="heading finance-solde-total__label">Entrées</p>
                  <p className="title compte-stats__value positif">{fmt(entrees)}</p>
                </div>
                <div className="column has-text-centered">
                  <p className="heading finance-solde-total__label">Sorties</p>
                  <p className="title compte-stats__value negatif">{fmt(sorties)}</p>
                </div>
              </div>

              <div className="tabs fin-tabs is-fullwidth">
                <ul>
                  <li className={tab === 'transactions' ? 'is-active' : ''}>
                    <a onClick={() => setTab('transactions')}>
                      Mouvements
                      {txs.length > 0 && <span className="tag fin-tab__badge">{txs.length}</span>}
                    </a>
                  </li>
                  <li className={tab === 'planned' ? 'is-active' : ''}>
                    <a onClick={() => setTab('planned')}>
                      Planifiées
                      {planned.length > 0 && <span className="tag fin-tab__badge">{planned.length}</span>}
                    </a>
                  </li>
                </ul>
              </div>

              {tab === 'transactions' && (
                txs.length === 0
                  ? <div className="notification finance-empty">Aucun mouvement  clique sur + 💸</div>
                  : <div className="fin-list">
                      {txs.map(t => (
                        <TransactionCard key={t.id} transaction={t} financeTypes={financeTypes} />
                      ))}
                    </div>
              )}

              {tab === 'planned' && (
                planned.length === 0
                  ? <div className="notification finance-empty">Aucune dépense planifiée  clique sur + 🔁</div>
                  : <div className="fin-list">
                      {planned.map(e => (
                        <PlannedExpenseCard key={e.id} expense={e} financeTypes={financeTypes} />
                      ))}
                    </div>
              )}

            </div>
          </div>
        </div>
      </section>

      {showTxModal && (
        <TransactionModal
          accountId={accountId}
          accounts={accounts}
          financeTypes={financeTypes}
          onClose={() => setShowTxModal(false)}
        />
      )}
      {showFacModal && (
        <PlannedExpenseModal
          accountId={accountId}
          accounts={accounts}
          financeTypes={financeTypes}
          onClose={() => setShowFacModal(false)}
        />
      )}
    </>
  )
}
