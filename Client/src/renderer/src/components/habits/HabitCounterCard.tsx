import type { Habit } from '@/types'

interface Props {
  habit:       Habit
  onIncrement: (id: string, value: number) => void
  onDelete:    (id: string) => void
}

export default function HabitCounterCard({ habit, onIncrement, onDelete }: Props) {
  const today    = new Date().toISOString().split('T')[0]
  const entry    = habit.entries.find((e) => e.date === today)
  const current  = entry?.value ?? 0
  const goal     = habit.goal ?? 100
  const step     = habit.step ?? 1
  // Calculer la progression : limite à 100% pour éviter un dépassement visuel
  const progress = Math.min((current / goal) * 100, 100)
  const isDone   = current >= goal

  return (
    // Conteneur de carte habitude avec progression: affiche l'état de complétion
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
          <span>·</span>
    <span className={`habit-reward ${habit.reward === 1 ? 'positive' : 'negative'}`}>
      {habit.reward === 1 ? '+ XP' : '− XP'}
    </span>     
     <span>·</span>
          <span className="habit-counter__goal">{current} / {goal} {habit.unit}</span>
        </div>
      </div>

      {/* media-right = compteur +/- */}
      <div className="media-right is-flex is-flex-direction-column" style={{ gap: '.375rem' }}>

        {/* field has-addons Bulma pour les boutons +/- collés */}
        <div className="field has-addons mb-0">
          <div className="control">
            <button className="button habit-counter__btn"
              onClick={() => onIncrement(habit.id, Math.max(0, current - step))}>
              −
            </button>
          </div>
          <div className="control">
            <button className="button habit-counter__val-btn" disabled>
              <span className="habit-counter__val">{current}</span>
            </button>
          </div>
          <div className="control">
            <button className="button habit-counter__btn"
              onClick={() => onIncrement(habit.id, current + step)}>
              +
            </button>
          </div>
        </div>

        <p className="has-text-centered habit-counter__unit">{habit.unit}</p>
      </div>
      <button className="delete is-small" onClick={() => onDelete(habit.id)} title="Supprimer" />
       
      {/* Mini barre absolue en bas  reste custom (position absolute) */}
      <div className="habit-bar-track">
        <div className="habit-bar-fill"
          style={{ width: `${progress}%`, background: habit.color, boxShadow: `0 0 6px ${habit.color}` }} />
      </div>

    </div>
  )
}