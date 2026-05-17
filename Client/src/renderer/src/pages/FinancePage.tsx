import { useState } from 'react'
import { useFinanceStore } from '@/store/useFinanceStore'
import { useLoadFinance }  from '@/hooks/useLoadFinance'
import { useUserStore }    from '@/store/userStore'
import CompteCard          from '../components/finance/CompteCard'
import CompteModal         from '../components/finance/CompteModal'
import CompteDetail        from '../components/finance/CompteDetail'
import 'bulma/css/bulma.min.css'
import '@/styles/finance.css'
import BackButton from '@/components/BackButton'

type View = { screen: 'list' } | { screen: 'detail'; accountId: number }

export default function FinancePage() {
  const userId = useUserStore(s => s.profile?.id) ?? 0
  const { accounts, transactions } = useFinanceStore()

  const { loading, error } = useLoadFinance(userId)

  const [view,      setView]      = useState<View>({ screen: 'list' })
  const [showModal, setShowModal] = useState(false)

  if (view.screen === 'detail') return (
    <CompteDetail
      accountId={view.accountId}
      onBack={() => setView({ screen: 'list' })}
    />
  )

  const soldeTotal = accounts.reduce((acc, a) => acc + a.balance, 0)

  return (
    <>
      <section className="hero is-fullheight finance-bg">
        <div className="hero-body is-align-items-flex-start" style={{ paddingTop: '2rem' }}>
          <div className="container">
            <div className="finance-page-header">
              <div className="level is-mobile mb-0">
                <div className="level-left">
                  <div className="level-item" style={{ gap: '.75rem' }}>
                    <BackButton to="/" label="←" />
                    <div>
                      <p className="finance-title">Finances</p>
                      <p className="finance-subtitle">Mes comptes</p>
                    </div>
                  </div>
                </div>
                <div className="level-right">
                  <div className="level-item">
                    <button className="button finance-add-btn" onClick={() => setShowModal(true)}>+</button>
                  </div>
                </div>
              </div>
            </div>

            <div className="box finance-box">

              <div className="finance-solde-total has-text-centered">
                <p className="heading finance-solde-total__label">Solde total</p>
                <p className={`title finance-solde-total__value ${soldeTotal >= 0 ? 'positif' : 'negatif'}`}>
                  {soldeTotal >= 0 ? '+' : ''}{soldeTotal.toLocaleString('fr-FR', { style: 'currency', currency: 'EUR' })}
                </p>
              </div>

              {loading && <div className="notification finance-empty">Chargement...</div>}
              {error   && <div className="notification is-danger">{error}</div>}

              {!loading && accounts.length === 0 && (
                <div className="notification finance-empty">
                  Aucun compte  clique sur + pour commencer 🏦
                </div>
              )}

              {!loading && accounts.length > 0 && (
                <div className="columns is-multiline is-mobile">
                  {accounts.map(a => (
                    <div key={a.id} className="column is-half">
                      <CompteCard
                        account={a}
                        transactions={transactions}
                        onClick={id => setView({ screen: 'detail', accountId: id })}
                        onDelete={() => {}}
                      />
                    </div>
                  ))}
                </div>
              )}

            </div>
          </div>
        </div>
      </section>

      {showModal && <CompteModal onClose={() => setShowModal(false)} />}
    </>
  )
}
