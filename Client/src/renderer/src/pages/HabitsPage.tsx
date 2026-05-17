import { useState } from 'react'
import 'bulma/css/bulma.min.css'
import '@/styles/habits.css'
import BackButton from '@/components/BackButton'
import { useHabitStore } from '@/store'
import HabitCheckCard   from '@/components/habits/HabitCheckCard'
import HabitCounterCard from '@/components/habits/HabitCounterCard'
import HabitModal       from '@/components/habits/HabitModal'

export default function HabitsPage() {
  const { habits, addHabit, logEntry, deleteHabit} = useHabitStore()
  const [showModal, setShowModal] = useState(false)

  const today = new Date().toISOString().split('T')[0]

  // Calcul des points habitudes
  const maxHabitPoints = habits
    .filter((h) => h.reward === 1)
    .length
  const earnedHabitPoints = habits
    .filter((h) => {
      const entry = h.entries.find((e) => e.date === today)
      if (!entry) return false
      if (h.type === 'counter') return entry.value >= (h.goal ?? 100)
      return entry.value > 0
    })
    .reduce((sum, h) => sum + h.reward, 0)
  const progress = maxHabitPoints > 0
    ? Math.round((earnedHabitPoints / maxHabitPoints) * 100) : 0

  function handleToggle(id: string) {
    const habit    = habits.find((h) => h.id === id)!
    const existing = habit.entries.find((e) => e.date === today)
    const newValue = (existing?.value ?? 0) > 0 ? 0 : 1
    logEntry(id, { date: today, value: newValue })
  }

  function handleIncrement(id: string, value: number) {
    logEntry(id, { date: today, value })
  }

  const todayLabel = new Date().toLocaleDateString('fr-FR', {
    weekday: 'long', day: 'numeric', month: 'long',
  })

  return (
    <>
      {/* Barre d'en-tête : navigation et bouton d'ajout */}
      <div className="level is-mobile mb-0 page-header">
        <div className="level-left">
          <div className="level-item" style={{ gap: '.75rem' }}>
            {/* Bouton retour vers accueil */}
            <BackButton to="/" label="←" />
            <div>
              <p className="habits-title">Habitudes</p>
              <p className="habits-date">{todayLabel}</p>
            </div>
          </div>
        </div>
        <div className="level-right">
          <div className="level-item">
            <button className="button habits-add-btn" onClick={() => setShowModal(true)}>+</button>
          </div>
        </div>
      </div>

      {/* Contenu avec padding-top pour compenser le header fixe */}
      <div className="page-content">

      {/* Barre de progression : points accumulés et pourcentage */}
      <div className="habits-progress">
        <div className="is-flex is-justify-content-space-between mb-1">
          <span className="is-size-7 has-text-weight-semibold habits-progress__left">
            {earnedHabitPoints}/{maxHabitPoints} pts aujourd'hui
          </span>
          <span className="is-size-7 has-text-weight-semibold habits-progress__right">
            {progress}%
          </span>
        </div>
        <div className="habits-progress__track">
          <div className="habits-progress__fill" style={{ width: `${progress}%` }} />
        </div>
      </div>

      {/* Liste des habitudes : affichage par type */}
      <div className="habits-list">
        {habits.length === 0 ? (
          // Message vide si aucune habitude
          <div className="notification habits-empty">
            Aucune habitude  clique sur + pour commencer 🌱
          </div>
        ) : (
          habits.map((habit) =>
            habit.type === 'check' ? (
              <HabitCheckCard key={habit.id} habit={habit}
                onToggle={handleToggle}
                onDelete={deleteHabit} />
            ) : (
              <HabitCounterCard key={habit.id} habit={habit}
                onIncrement={handleIncrement}
                onDelete={deleteHabit} />
            )
          )
        )}
      </div>

      {/* Modal d'ajout/modification : rendu conditionnel */}
      {showModal && (
        <HabitModal onClose={() => setShowModal(false)} onSave={addHabit} />
      )}
      </div>
    </>
  )
}