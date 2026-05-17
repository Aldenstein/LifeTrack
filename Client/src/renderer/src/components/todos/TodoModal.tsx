import { useState } from 'react'
import { useTodoStore } from '@/store'
import type { Todo, TodoPriority } from '@/types'
import { PRIORITY_CONFIG } from '@/constants/todoPriority'

interface Props {
  todo:    Todo | null
  onClose: () => void
}

export default function TodoModal({ todo, onClose }: Props) {
  const { addTodo, updateTodo } = useTodoStore()

  // Formulaire de saisie : pré-remplit avec les données existantes ou valeurs par défaut
  const [form, setForm] = useState({
    title:       todo?.title            ?? '',
    description: todo?.description      ?? '',
    priority:    todo?.priority         ?? 'medium' as TodoPriority,
    tag:         todo?.tag              ?? '',
    dueDate:     todo?.dueDate          ?? '',
    timer:       todo?.timer            ?? null,
  })

  // Setter générique pour mettre à jour le formulaire avec typage
  function set<K extends keyof typeof form>(key: K, val: typeof form[K]) {
    setForm(f => ({ ...f, [key]: val }))
  }

  // Valider et soumettre: créer ou modifier selon le contexte
  function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    const payload = {
      title:       form.title.trim(),
      description: form.description.trim() || undefined,
      priority:    form.priority,
      tag:         form.tag.trim()  || undefined,
      dueDate:     form.dueDate     || undefined,
      timer:       form.timer,
      timerElapsed: form.timer === null ? 0 : todo?.timerElapsed ?? 0,
      timerRunning: false,
    }
    if (todo) { updateTodo(todo.id, payload) }
    else { addTodo({ ...payload, id: crypto.randomUUID(), completed: false, createdAt: new Date().toISOString() }) }
    onClose()
  }

  return (
    // Modal Bulma avec fond de fermeture
    <div className="modal is-active todo-modal-wrap">
      <div className="modal-background" onClick={onClose} />

      <div className="modal-card todo-modal-card">

        {/* En-tête du modal */}
        <header className="modal-card-head todo-modal-head">
          <p className="modal-card-title todo-modal__title">
            {todo ? 'Modifier la tâche' : 'Nouvelle tâche'}
          </p>
          <button className="delete" onClick={onClose} aria-label="Fermer" />
        </header>

        {/* Corps du formulaire */}
        <section className="modal-card-body todo-modal-body">
          <form id="todo-form" onSubmit={handleSubmit}>

            {/* Champ titre obligatoire */}
            <div className="field">
              <label className="label todo-label">Titre *</label>
              <div className="control">
                <input className="input todo-input" required
                  placeholder="Ex : Finir le dashboard..."
                  value={form.title} onChange={e => set('title', e.target.value)} />
              </div>
            </div>

            {/* Champ description optionnel */}
            <div className="field">
              <label className="label todo-label">Description</label>
              <div className="control">
                <textarea className="textarea todo-input todo-textarea" rows={2}
                  placeholder="Détails optionnels..."
                  value={form.description} onChange={e => set('description', e.target.value)} />
              </div>
            </div>

            {/* Sélecteurs de priorité et tag */}
            <div className="columns is-mobile is-variable is-2">
              <div className="column">
                <div className="field">
                  <label className="label todo-label">Priorité</label>
                  <div className="control">
                    <div className="select is-fullwidth todo-select">
                      <select value={form.priority} onChange={e => set('priority', e.target.value as TodoPriority)}>
                        {(Object.entries(PRIORITY_CONFIG) as [TodoPriority, { label: string; color: string }][])
                          .map(([k, v]) => <option key={k} value={k}>{v.label}</option>)}
                      </select>
                    </div>
                  </div>
                </div>
              </div>
              <div className="column">
                <div className="field">
                  <label className="label todo-label">Tag</label>
                  <div className="control">
                    <input className="input todo-input" placeholder="Dev, École, Perso..."
                      value={form.tag} onChange={e => set('tag', e.target.value)} />
                  </div>
                </div>
              </div>
            </div>

            {/* Sélecteur de date d'échéance */}
            <div className="field">
              <label className="label todo-label">Date d'échéance</label>
              <div className="control">
                <input className="input todo-input" type="date"
                  value={form.dueDate} onChange={e => set('dueDate', e.target.value)} />
              </div>
            </div>

            {/* Minuteur optionnel avec saisie minute */}
            <div className="field">
              <label className="label todo-label">Minuteur (minutes)</label>
              <div className="control">
                <input
                  className="input todo-input"
                  type="number"
                  min={1}
                  max={480}
                  placeholder="Aucun minuteur"
                  value={form.timer ?? ''}
                  onChange={e => set('timer', e.target.value === '' ? null : Number(e.target.value))}
                />
              </div>
              <p className="help todo-timer-config__hint">Laisser vide pour ne pas définir de minuteur.</p>
            </div>

          </form>
        </section>

        {/* Pied du modal avec boutons d'action */}
        <footer className="modal-card-foot todo-modal-foot">
          <div className="buttons is-right" style={{ width: '100%' }}>
            <button type="button" className="button todo-btn-cancel" onClick={onClose}>Annuler</button>
            <button type="submit" form="todo-form" className="button todo-btn-submit">
              {todo ? 'Sauvegarder' : 'Créer'}
            </button>
          </div>
        </footer>

      </div>
    </div>
  )
}
