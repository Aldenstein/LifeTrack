import type { Habit } from '@/types'

interface Props {
  habit:    Habit
  onToggle: (id: string) => void
  onDelete: (id: string) => void
}

export default function HabitCheckCard({ habit, onToggle, onDelete }: Props) {
  const today  = new Date().toISOString().split('T')[0]
  // Vérifier si l'habitude est déjà completée pour aujourd'hui
  const isDone = habit.entries.some((e) => e.date === today && e.value > 0)

  return (
    // Conteneur de carte habitude : affiche l'état de complétion
    <div className={`media habit-card${isDone ? ' done' : ''}`}>

      {/* Icône : visuellement distinctif par couleur */}
      <div className="media-left">
        <div className="habit-icon" style={{ background: `${habit.color}18` }}>
          {habit.icon}
        </div>
      </div>

      {/* Informations : nom, streak, récompense */}
      <div className="media-content">
        <p className="habit-name">{habit.name}</p>
        <div className="habit-meta">
          <span className="habit-streak">🔥 {habit.streak}</span>
          <span className={`habit-reward ${habit.reward === 1 ? 'positive' : 'negative'}`}>
          {habit.reward === 1 ? '+ XP' : '− XP'}
        </span>
          <span>·</span>
          <span>{isDone ? "Fait aujourd'hui" : 'À faire'}</span>
        </div>
      </div>

      {/* media-right = actions */}
      <div className="media-right is-flex is-align-items-center" style={{ gap: '.5rem' }}>
        {/* delete Bulma remplace habit-delete */}
        <button
          className={`button habit-check-btn${isDone ? ' checked' : ''}`}
          onClick={() => onToggle(habit.id)}
          title={isDone ? 'Décocher' : 'Valider'}
        >
          {isDone ? '✓' : ''}
        </button>
        <button className="delete is-small" onClick={() => onDelete(habit.id)} title="Supprimer" />

      </div>

    </div>
  )
}