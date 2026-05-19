import { useState, useEffect, useRef } from 'react'
import { useTodoStore } from '@/store'
import type { Todo } from '@/types'
import { PRIORITY_CONFIG } from '@/constants/todoPriority'

interface Props {
  todo:   Todo
  onEdit: (todo: Todo) => void
}

function formatTime(seconds: number): string {
  const m = Math.floor(seconds / 60).toString().padStart(2, '0')
  const s = (seconds % 60).toString().padStart(2, '0')
  return `${m}:${s}`
}

export default function TodoCard({ todo, onEdit }: Props) {
  // Correction : toggleComplete et deleteTodo appellent l'API via le store
  // updateTodo reste local pour le timer (pas de route PATCH todo dans l'API)
  const { toggleComplete, deleteTodo, updateTodo } = useTodoStore()
  const [exiting, setExiting] = useState(false)
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null)

  const priority  = PRIORITY_CONFIG[todo.priority]
  const isOverdue = todo.dueDate && !todo.completed && new Date(todo.dueDate) < new Date()
  const timer     = todo.timer
  const elapsed   = todo.timerElapsed ?? 0

  useEffect(() => {
    if (!timer || !todo.timerRunning) {
      if (intervalRef.current) clearInterval(intervalRef.current)
      return
    }
    const totalSeconds = timer * 60
    intervalRef.current = setInterval(() => {
      const nextElapsed = Math.min(elapsed + 1, totalSeconds)
      updateTodo(todo.id, {
        timerElapsed: nextElapsed,
        timerRunning: nextElapsed < totalSeconds,
      })
    }, 1000)
    return () => { if (intervalRef.current) clearInterval(intervalRef.current) }
  }, [timer, todo.timerRunning, elapsed])

  function toggleTimer() {
    if (!timer) return
    updateTodo(todo.id, { timerRunning: !todo.timerRunning })
  }

  function resetTimer() {
    if (!timer) return
    updateTodo(todo.id, { timerElapsed: 0, timerRunning: false })
  }

  function handleDelete() {
    setExiting(true)
    // Correction : deleteTodo appelle POST /users/{id}/todos avec completed=true
    // (pas de route DELETE dans l'API — suppression locale uniquement comme pour finance)
    setTimeout(() => deleteTodo(todo.id), 280)
  }

  const totalSeconds = (timer ?? 0) * 60
  const remaining    = Math.max(totalSeconds - elapsed, 0)
  const displayTime  = timer ? formatTime(remaining) : null
  const progressPct  = timer ? Math.min((elapsed / totalSeconds) * 100, 100) : 0

  return (
    <div className={`box todo-card${todo.completed ? ' todo-card--done' : ''}${exiting ? ' todo-card--exit' : ''}`}>

      <div className="media todo-card__main">

        <div className="media-left">
          <button
            className={`todo-card__check${todo.completed ? ' todo-card__check--done' : ''}`}
            // Correction : toggleComplete appelle POST /users/{id}/todos/{id}/complete
            onClick={() => toggleComplete(todo.id)}
            aria-label="Marquer comme termine"
          >
            {todo.completed ? '✓' : ''}
          </button>
        </div>

        <div className="media-content">
          <p className="todo-card__title">{todo.title}</p>
          {todo.description && <p className="todo-card__desc">{todo.description}</p>}
          <div className="tags todo-card__meta mb-0">
            <span className="tag todo-card__badge"
              style={{ color: priority.color, borderColor: priority.color }}>
              {priority.label}
            </span>
            {todo.tag && <span className="tag todo-card__tag">#{todo.tag}</span>}
            {todo.dueDate && (
              <span className={`tag${isOverdue ? ' todo-card__due--overdue' : ' todo-card__due'}`}>
                📅 {new Date(todo.dueDate).toLocaleDateString('fr-FR')}
              </span>
            )}
          </div>
        </div>

        <div className="media-right todo-card__actions">
          <div className="buttons are-small">
            <button className="button todo-card__btn" onClick={() => onEdit(todo)} title="Modifier">✏️</button>
            <button className="button todo-card__btn todo-card__btn--danger" onClick={handleDelete} title="Supprimer">🗑️</button>
          </div>
        </div>

      </div>

      {timer && (
        <>
          <div className="todo-card__timer">
            <div>
              <div className="todo-card__timer-display">{displayTime}</div>
              <div className="todo-card__timer-mode">Minuteur {timer} min</div>
            </div>
            <div className="buttons mb-0 todo-card__timer-controls">
              <button
                className={`button todo-card__timer-btn${todo.timerRunning ? ' is-active' : ''}`}
                onClick={toggleTimer}
                title={todo.timerRunning ? 'Pause' : 'Demarrer'}>
                {todo.timerRunning ? '⏸' : '▶️'}
              </button>
              <button className="button todo-card__timer-btn" onClick={resetTimer} title="Reinitialiser">
                🔄
              </button>
            </div>
          </div>
          <div className="todo-card__timer-progress">
            <div className="todo-card__timer-progress-fill" style={{ width: `${progressPct}%` }} />
          </div>
        </>
      )}

    </div>
  )
}
