import { useState, useMemo } from 'react'
import '@/styles/todos.css'
import 'bulma/css/bulma.min.css'
import BackButton    from '@/components/BackButton'
import { useTodoStore } from '@/store'
import type { Todo } from '@/types'
import TodoCard  from '@/components/todos/TodoCard'
import TodoModal from '@/components/todos/TodoModal'

type Filter = 'all' | 'active' | 'done'

export default function TodosPage() {
  const { todos } = useTodoStore()

  const [showModal, setShowModal] = useState(false)
  const [editTodo,  setEditTodo]  = useState<Todo | null>(null)
  const [filter,    setFilter]    = useState<Filter>('all')

  const doneCount  = todos.filter(t =>  t.completed).length
  const totalCount = todos.length
  const progress   = totalCount > 0 ? Math.round((doneCount / totalCount) * 100) : 0

  const todayLabel = new Date().toLocaleDateString('fr-FR', {
    weekday: 'long', day: 'numeric', month: 'long',
  })

  const filtered = useMemo(() =>
    todos.filter(t => {
      if (filter === 'active') return !t.completed
      if (filter === 'done')   return  t.completed
      return true
    }),
    [todos, filter]
  )

  function openCreate()      { setEditTodo(null); setShowModal(true) }
  function openEdit(t: Todo) { setEditTodo(t);    setShowModal(true) }

  return (
    <>
      <div className="level is-mobile mb-0 page-header">
        <div className="level-left">
          <div className="level-item" style={{ gap: '.75rem' }}>
            <BackButton to="/" label="←" />
            <div>
              <p className="todos-title">To-Do</p>
              <p className="todos-date">{todayLabel}</p>
            </div>
          </div>
        </div>
        <div className="level-right">
          <div className="level-item">
            <button className="button todos-add-btn" onClick={openCreate}>+</button>
          </div>
        </div>
      </div>

      <div className="page-content">

        <div className="todos-progress">
          <div className="is-flex is-justify-content-space-between mb-1">
            <span className="is-size-7 todos-progress__left">{doneCount}/{totalCount} completees</span>
            <span className="is-size-7 has-text-weight-semibold todos-progress__right">{progress}%</span>
          </div>
          <div className="todos-progress__track">
            <div className="todos-progress__fill" style={{ width: `${progress}%` }} />
          </div>
        </div>

        <div className="tabs is-toggle is-fullwidth todos-filter-tabs">
          <ul>
            {(['all', 'active', 'done'] as Filter[]).map(f => (
              <li key={f} className={filter === f ? 'is-active' : ''}>
                <a onClick={() => setFilter(f)}>
                  {{ all: 'Tout', active: 'Actives', done: 'Terminees' }[f]}
                </a>
              </li>
            ))}
          </ul>
        </div>

        <div className="todos-list">
          {filtered.length === 0 ? (
            <div className="notification todos-empty">
              {filter === 'done'
                ? 'Aucune tache terminee 💪'
                : 'Clique sur + pour commencer ✅'}
            </div>
          ) : (
            filtered.map(todo => (
              <TodoCard key={todo.id} todo={todo} onEdit={openEdit} />
            ))
          )}
        </div>

        {showModal && (
          <TodoModal todo={editTodo} onClose={() => setShowModal(false)} />
        )}

      </div>
    </>
  )
}
