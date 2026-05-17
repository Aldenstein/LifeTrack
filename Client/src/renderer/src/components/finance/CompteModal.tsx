import { useState } from 'react'
import { useFinanceStore } from '@/store/useFinanceStore'
import { financeService } from '@/services/financeService'
import { useUserStore } from '@/store/userStore'

interface Props { onClose: () => void }

export default function CompteModal({ onClose }: Props) {
  const userId              = useUserStore(s => s.profile?.id)
  const { addAccount }      = useFinanceStore()

  const [name,    setName]    = useState('')
  const [balance, setBalance] = useState('')
  const [error,   setError]   = useState<string | null>(null)
  const [loading, setLoading] = useState(false)

  const valide = name.trim().length > 0

  async function handleSave() {
    if (!valide || !userId) return
    try {
      setLoading(true)
      const { id } = await financeService.createAccount(userId, {
        name:    name.trim(),
        balance: parseFloat(balance) || 0,
      })
      addAccount({ id, name: name.trim(), balance: parseFloat(balance) || 0 })
      onClose()
    } catch (err: any) {
      setError(err.message)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="modal is-active finance-modal">
      <div className="modal-background" onClick={onClose} />
      <div className="modal-card finance-modal__card">
        <header className="modal-card-head finance-modal__head">
          <p className="modal-card-title finance-modal__title">Nouveau compte</p>
          <button className="delete" onClick={onClose} />
        </header>
        <section className="modal-card-body finance-modal__body">
          {error && <p className="help is-danger">{error}</p>}
          <div className="field">
            <label className="label finance-label">Nom du compte</label>
            <div className="control">
              <input className="input finance-input" type="text" placeholder="Ex : Compte courant"
                value={name} onChange={e => setName(e.target.value)} autoFocus />
            </div>
          </div>
          <div className="field">
            <label className="label finance-label">Solde initial (€)</label>
            <div className="control has-icons-left">
              <input className="input finance-input" type="number" step="0.01" placeholder="0.00"
                value={balance} onChange={e => setBalance(e.target.value)} />
              <span className="icon is-left finance-input__icon">€</span>
            </div>
          </div>
        </section>
        <footer className="modal-card-foot finance-modal__foot">
          <div className="buttons is-right" style={{ width: '100%' }}>
            <button className="button finance-btn-cancel" onClick={onClose}>Annuler</button>
            <button className="button finance-btn-submit" onClick={handleSave}
              disabled={!valide || loading}>
              {loading ? 'Création...' : 'Créer'}
            </button>
          </div>
        </footer>
      </div>
    </div>
  )
}
