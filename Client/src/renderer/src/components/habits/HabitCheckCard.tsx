import type { Habit } from '@/types'

interface Props {
  habit:    Habit
  onToggle: (id: string) => void
  onDelete: (id: string) => void
}

export default function HabitCheckCard({ habit, onToggle, onDelete }: Props) {
  const today  = new Date().toISOString().split('T')[0]
  // Verifie si l'habitude est deja completee pour aujourd'hui
  const isDone = habit.entries.some((e) => e.date === today && e.value > 0)

  return (
    <div className={`media habit-card${isDone ? ' done' : ''}`}>

      {/* Icone */}
      <div className="media-left">
        <div className="habit-icon" style={{ background: `${habit.color}18` }}>
          {habit.icon}
        </div>
      </div>

      {/* Infos */}
      <div className="media-content">
        <p className="habit-name">{habit.name}</p>
        <div className="habit-meta">
          <span className="habit-streak">🔥 {habit.streak}</span>
          <span className={`habit-reward ${habit.reward === 1 ? 'positive' : 'negative'}`}>
            {habit.reward === 1 ? '+ XP' : '- XP'}
          </span>
          <span>·</span>
          <span>{isDone ? "Fait aujourd'hui" : 'A faire'}</span>
        </div>
      </div>

      {/* Actions */}
      <div className="media-right is-flex is-align-items-center" style={{ gap: '.5rem' }}>
        <button
          className={`button habit-check-btn${isDone ? ' checked' : ''}`}
          // Correction : onToggle appelle completeHabit dans le store
          // qui appelle POST /habits/{id}/complete avec la date du jour
          onClick={() => onToggle(habit.id)}
          title={isDone ? 'Decocher' : 'Valider'}
        >
          {isDone ? '✓' : ''}
        </button>
        <button className="delete is-small" onClick={() => onDelete(habit.id)} title="Supprimer" />
      </div>

    </div>
  )
}
