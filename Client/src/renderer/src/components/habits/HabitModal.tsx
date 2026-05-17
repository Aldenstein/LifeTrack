import { useState } from 'react'
import type { Habit, HabitType } from '@/types'

interface Props {
  onClose: () => void
  onSave:  (habit: Habit) => void
}

const COLORS = ['#9d93e0','#7ec8a4','#f0c070','#e07c9d','#70b8f0','#f07c70']
const ICONS  = ['🧘','💧','🏃','📚','🎯','💪','🥗','😴','✍️','🎵']

export default function HabitModal({ onClose, onSave }: Props) {
  const [name,  setName]  = useState('')
  const [icon,  setIcon]  = useState('🎯')
  const [color, setColor] = useState('#9d93e0')
  const [type,  setType]  = useState<HabitType>('check')
  const [unit,  setUnit]  = useState('')
  const [goal,  setGoal]  = useState('100')
  const [step,  setStep]  = useState('1')
  const [reward, setReward] = useState<1 | -1>(1)

  function handleSave() {
    if (!name.trim()) return
    const habit: Habit = {
      id: crypto.randomUUID(), name: name.trim(),
      icon, color, type, frequency: 'daily',
      entries: [], streak: 0, reward: reward,
      createdAt: new Date().toISOString(),
      ...(type === 'counter' && { unit: unit || 'fois', goal: Number(goal), step: Number(step) }),
    }
    onSave(habit)
    onClose()
  }

  return (
    /* modal Bulma remplace modal-overlay custom */
    <div className="modal is-active habit-modal">
      {/* modal-background gère le fond flouté + clic fermer */}
      <div className="modal-background" onClick={onClose} />

      <div className="modal-card habit-modal__card">

        {/* modal-card-head Bulma = header */}
        <header className="modal-card-head">
          <p className="modal-card-title">Nouvelle habitude</p>
          {/* delete Bulma = bouton fermer */}
          <button className="delete" onClick={onClose} aria-label="Fermer" />
        </header>

        {/* modal-card-body Bulma = contenu scrollable */}
        <section className="modal-card-body">

          {/* Nom */}
          <div className="field">
            <label className="label">Nom</label>
            <div className="control">
              <input className="input" placeholder="Ex: Méditation"
                value={name} onChange={(e) => setName(e.target.value)} autoFocus />
            </div>
          </div>

          {/* Icône */}
          <div className="field">
            <label className="label">Icône</label>
            <div className="is-flex is-flex-wrap-wrap" style={{ gap: '.5rem' }}>
              {ICONS.map((i) => (
                <button key={i} onClick={() => setIcon(i)} className={`button habit-icon-btn${icon === i ? ' is-active' : ''}`}>
                  {i}
                </button>
              ))}
            </div>
          </div>

          {/* Couleur */}
          <div className="field">
            <label className="label">Couleur</label>
            <div className="is-flex" style={{ gap: '.5rem' }}>
              {COLORS.map((c) => (
                <button key={c} onClick={() => setColor(c)}
                  className="habit-color-btn"
                  style={{ background: c, outline: color === c ? `3px solid ${c}` : 'none', outlineOffset: 2 }}
                />
              ))}
            </div>
          </div>

          {/* Type  tabs is-toggle Bulma */}
          <div className="field">
            <label className="label">Type</label>
            <div className="tabs is-toggle is-fullwidth habit-type-tabs">
              <ul>
                <li className={type === 'check' ? 'is-active' : ''}>
                  <a onClick={() => setType('check')}>✓ Checkbox</a>
                </li>
                <li className={type === 'counter' ? 'is-active' : ''}>
                  <a onClick={() => setType('counter')}>🔢 Compteur</a>
                </li>
              </ul>
            </div>
          </div>

          {/* Champs compteur  columns Bulma remplace modal-row */}
          {type === 'counter' && (
            <div className="habit-counter-fields">
              <div className="columns is-mobile">
                <div className="column">
                  <div className="field">
                    <label className="label">Objectif</label>
                    <div className="control">
                      <input className="input" type="number" value={goal}
                        onChange={(e) => setGoal(e.target.value)} />
                    </div>
                  </div>
                </div>
                <div className="column">
                  <div className="field">
                    <label className="label">Unité</label>
                    <div className="control">
                      <input className="input" placeholder="ml, reps…" value={unit}
                        onChange={(e) => setUnit(e.target.value)} />
                    </div>
                  </div>
                </div>
              </div>
              <div className="field">
                <label className="label">Incrément rapide</label>
                <div className="control">
                  <input className="input" type="number" value={step}
                    onChange={(e) => setStep(e.target.value)} />
                </div>
              </div>
            </div>
          )}
      <ul>
            <li className={reward === 1 ? 'is-active' : ''}>
              <a onClick={() => setReward(1)}>✅ Positif</a>
            </li>
            <li className={reward === -1 ? 'is-active' : ''}>
              <a onClick={() => setReward(-1)}>❌ Négatif</a>
            </li>
          </ul>
        </section>

        {/* modal-card-foot Bulma = footer avec submit */}
        <footer className="modal-card-foot">
          <button className="button habit-submit is-fullwidth" onClick={handleSave}>
            Ajouter l'habitude
          </button>
        </footer>

      </div>
    </div>
  )
}