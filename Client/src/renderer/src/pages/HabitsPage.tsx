// pages/HabitsPage.tsx
import { useState } from 'react'
import 'bulma/css/bulma.min.css'
import '@/styles/habits.css'
import BackButton       from '@/components/BackButton'
import { useHabitStore } from '@/store'
import HabitCheckCard   from '@/components/habits/HabitCheckCard'
import HabitCounterCard from '@/components/habits/HabitCounterCard'
import HabitModal       from '@/components/habits/HabitModal'

export default function HabitsPage() {
  const { habits, addHabit, completeHabit, updateCounter, deleteHabit } = useHabitStore()
  const [showModal, setShowModal] = useState(false)

  const today = new Date().toISOString().split('T')[0]

  const maxHabitPoints = habits.filter((h) => h.reward === 1).length
  const earnedHabitPoints = habits
    .filter((h) => {
      const entry = h.entries.find((e) => e.date === today)
      if (!entry) return false
      if (h.type === 'counter') return entry.value >= (h.goal ?? 100)
      return entry.value > 0
    })
    .reduce((sum, h) => sum + h.reward, 0)
  const progress = maxHabitPoints > 0
    ? Math.round((earnedHabitPoints / maxHabitPoints) * 100)
    : 0

  // Checkbox : complete via API habit
  function handleToggle(id: string) {
    const habit   = habits.find((h) => h.id === id)!
    const isDone  = habit.entries.some((e) => e.date === today && e.value > 0)
    if (!isDone) completeHabit(id, today)
  }

  // Compteur : persist via Zero-Knowledge (/encrypted)
  function handleIncrement(id: string, date: string, value: number) {
    updateCounter(id, date, value)
  }

  const todayLabel = new Date().toLocaleDateString('fr-FR', {
    weekday: 'long', day: 'numeric', month: 'long',
  })

  return (
    <>
      <div className="level is-mobile mb-0 page-header">
        <div className="level-left">
          <div className="level-item" style={{ gap: '.75rem' }}>
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

      <div className="page-content">

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

        <div className="habits-list">
          {habits.length === 0 ? (
            <div className="notification habits-empty">
              Aucune habitude — clique sur + pour commencer 🌱
            </div>
          ) : (
            habits.map((habit) =>
              habit.type === 'check' ? (
                <HabitCheckCard key={habit.id} habit={habit}
                  onToggle={handleToggle}
                  onDelete={deleteHabit} />
              ) : (
                // Correction : signature onIncrement(id, date, value)
                <HabitCounterCard key={habit.id} habit={habit}
                  onIncrement={handleIncrement}
                  onDelete={deleteHabit} />
              )
            )
          )}
        </div>

        {showModal && (
          <HabitModal onClose={() => setShowModal(false)} onSave={addHabit} />
        )}

      </div>
    </>
  )
}
