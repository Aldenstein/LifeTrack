import { useNavigate } from 'react-router-dom'
import { useHabitStore, useTodoStore } from '@/store'
import { useAuth } from '@/hooks/useAuth'
import { useUserStore } from '@/store/userStore'
import Gauge from './GaugeChart'
import Tiles from './Tiles'

export default function HomePanel() {
  const navigate = useNavigate()
  const { logout } = useAuth()
  const { profile } = useUserStore()
  const { habits } = useHabitStore()
  const { todos } = useTodoStore()

  const today = new Date().toISOString().split('T')[0]

  // Calcul des points habitudes : potentiel max = somme des rewards positifs
  const maxHabitPoints = habits
    .filter((h) => h.reward === 1)
    .length

  // Points gagnés = somme des rewards pour habitudes complétées (positif - négatif)
  // Dissociation: check = value > 0, counter = value >= goal
  const earnedHabitPoints = habits
    .filter((h) => {
      const entry = h.entries.find((e) => e.date === today)
      if (!entry) return false
      if (h.type === 'counter') return entry.value >= (h.goal ?? 100)
      return entry.value > 0
    })
    .reduce((sum, h) => sum + h.reward, 0)

  const totalTodos  = todos.length
  const doneTodos   = todos.filter((t) => t.completed).length
  const total       = maxHabitPoints + totalTodos
  const done        = earnedHabitPoints + doneTodos
  // Ratios de complétion pour habitudes et todos
  const habitsScore = maxHabitPoints > 0 ? earnedHabitPoints / maxHabitPoints : 0
  const todosScore  = totalTodos > 0 ? doneTodos / totalTodos : 0

  // Score global = moyenne pondérée : habitudes 25% + todos 25%
  const accomplishment = Math.round(
    (habitsScore * 0.25 + todosScore * 0.25) * 100
  )

  const tiles = [
    {
      id: 'habits', icon: '\u{1F501}', name: 'Habitudes',
      sub: `${earnedHabitPoints}/${maxHabitPoints} pts aujourd'hui`,
      progress: maxHabitPoints > 0 ? Math.round((earnedHabitPoints / maxHabitPoints) * 100) : 0,
      badge: `${earnedHabitPoints}/${maxHabitPoints}`, color: '#9d93e0', route: '/habits',
    },
    {
      id: 'todos', icon: '\u2705', name: 'To-Do',
      sub: `${doneTodos} complétées`,
      progress: totalTodos > 0 ? Math.round((doneTodos / totalTodos) * 100) : 0,
      badge: `${doneTodos}/${totalTodos}`, color: '#7ec8a4', route: '/todos',
    },
    {
      id: 'finance', icon: '\uD83D\uDCB0', name: 'Finance',
      sub: 'Budget du mois', progress: 0,
      badge: '', color: '#f0c070', route: '/finance',
    },
    {
      id: 'health', icon: '\u2764\uFE0F', name: 'Santé',
      sub: 'Objectif du jour', progress: 0,
      badge: '', color: '#e07c9d', route: '/health',
    },
  ]

  function handleLogout() {
    logout()
    navigate('/auth')
  }

  return (
    <>
      {/* Barre d'en-tête fixe */}
      <div className="level is-mobile mb-0 page-header">
        <div className="level-left">
          <div className="level-item">
            <div>
              <p className="is-size-7 has-text-weight-bold is-uppercase dash-sub">
                Bonjour 👋
              </p>
              <p className="dash-name">{profile?.username ?? 'Utilisateur'}</p>
            </div>
          </div>
        </div>
        <div className="level-right">
          <div className="level-item">
            <button className="button dash-logout" onClick={handleLogout} title="Déconnexion">
              ↩
            </button>
          </div>
        </div>
      </div>

      {/* Contenu principal avec padding pour le header fixe */}
      <div className="page-content">
        
              

                {/* Jauge de progression globale */}
                <div className="gauge-section">
                  <Gauge value={accomplishment} label="Accompli" size={200} />

                  {/* Statistiques : items complétés et en cours */}
                  <div className="columns is-mobile is-centered mt-0">
                    <div className="column is-narrow has-text-centered">
                      <p className="gstat__val">{done}</p>
                      <p className="is-size-7 has-text-weight-bold is-uppercase gstat__lbl">Faits</p>
                    </div>
                    <div className="column is-narrow has-text-centered">
                      <p className="gstat__val">{total - done}</p>
                      <p className="is-size-7 has-text-weight-bold is-uppercase gstat__lbl">Restants</p>
                    </div>
                  </div>
                </div>

              

                {/* Grille de tuiles pour accès rapide à chaque section */}
                <div className="columns is-multiline is-mobile is-2">
                  {tiles.map((tile) => (
                    <div key={tile.id} className="column is-half">
                      <Tiles {...tile} />
                    </div>
                  ))}
                </div>

              </div>
            
          
    </>
  )
}