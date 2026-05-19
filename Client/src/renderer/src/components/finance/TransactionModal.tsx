import { useState } from 'react'
import { useFinanceStore } from '@/store/useFinanceStore'
import { financeService }  from '@/services/financeService'
import { useUserStore }    from '@/store/userStore'
import type { ApiFinanceType, ApiAccount } from '@/types/api'

interface Props {
  accountId:    number
  accounts:     ApiAccount[]
  financeTypes: ApiFinanceType[]
  onClose:      () => void
}

type Direction = 'depense' | 'entree'

export default function TransactionModal({ accountId, accounts, financeTypes, onClose }: Props) {
  const userId             = useUserStore(s => s.profile?.id)
  const { addTransaction } = useFinanceStore()

  const [direction,   setDirection]   = useState<Direction>('depense')
  const [montant,     setMontant]     = useState('')
  const [description, setDescription] = useState('')
  const [date,        setDate]        = useState(new Date().toISOString().split('T')[0])
  const [typeId,      setTypeId]      = useState<number>(financeTypes[0]?.id ?? 0)
  const [comId,       setComId]       = useState<number>(accountId)
  const [error,       setError]       = useState<string | null>(null)
  const [loading,     setLoading]     = useState(false)

  const valide = parseFloat(montant) > 0 && description.trim().length > 0

  async function handleSave() {
    if (!valide || !userId) return
    const amount = direction === 'depense'
      ? -Math.abs(parseFloat(montant))
      :  Math.abs(parseFloat(montant))
    try {
      setLoading(true)
      const created = await financeService.createTransaction(userId, {
        account_id:  comId,
        type_id:     typeId,
        amount,
        description: description.trim(),
        date,
      })
      addTransaction(created)
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
          <p className="modal-card-title finance-modal__title">Nouveau mouvement</p>
          <button className="delete" onClick={onClose} />
        </header>
        <section className="modal-card-body finance-modal__body">
          {error && <p className="help is-danger">{error}</p>}
          <div className="field">
            <div className="buttons has-addons fin-toggle">
              {(['depense', 'entree'] as Direction[]).map(d => (
                <button key={d}
                  className={`button fin-toggle__btn ${direction === d ? 'is-active' : ''}`}
                  onClick={() => setDirection(d)}>
                  {d === 'depense' ? '- Depense' : '+ Entree'}
                </button>
              ))}
            </div>
          </div>
          <div className="field">
            <label className="label finance-label">Montant</label>
            <div className="control has-icons-left">
              <input className="input finance-input" type="number" min="0" step="0.01"
                placeholder="0.00" value={montant} onChange={e => setMontant(e.target.value)} autoFocus />
              <span className="icon is-left finance-input__icon">€</span>
            </div>
          </div>
          <div className="field">
            <label className="label finance-label">Description <span className="fin-required">*</span></label>
            <div className="control">
              <input className="input finance-input" type="text" placeholder="Ex : Courses Lidl"
                value={description} onChange={e => setDescription(e.target.value)} />
            </div>
          </div>
          <div className="field">
            <label className="label finance-label">Date</label>
            <div className="control">
              <input className="input finance-input" type="date"
                value={date} onChange={e => setDate(e.target.value)} />
            </div>
          </div>
          <div className="field">
            <label className="label finance-label">Type</label>
            <div className="control">
              <div className="select finance-select is-fullwidth">
                <select value={typeId} onChange={e => setTypeId(Number(e.target.value))}>
                  {financeTypes.map(t => <option key={t.id} value={t.id}>{t.name}</option>)}
                </select>
              </div>
            </div>
          </div>
          <div className="field">
            <label className="label finance-label">Compte</label>
            <div className="control">
              <div className="select finance-select is-fullwidth">
                <select value={comId} onChange={e => setComId(Number(e.target.value))}>
                  {accounts.map(a => <option key={a.id} value={a.id}>{a.name}</option>)}
                </select>
              </div>
            </div>
          </div>
        </section>
        <footer className="modal-card-foot finance-modal__foot">
          <div className="buttons is-right" style={{ width: '100%' }}>
            <button className="button finance-btn-cancel" onClick={onClose}>Annuler</button>
            <button className="button finance-btn-submit" onClick={handleSave}
              disabled={!valide || loading}>
              {loading ? 'Ajout...' : 'Ajouter'}
            </button>
          </div>
        </footer>
      </div>
    </div>
  )
}
