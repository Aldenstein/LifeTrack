import { useNavigate } from 'react-router-dom'

interface TileProps {
  id:       string
  icon:     string
  name:     string
  sub:      string
  progress: number
  badge?:    string
  color:    string
  route:    string
}

function getTileColor(progress: number, baseColor: string): string {
  if (progress >= 75) return '#7ec8a4'
  if (progress >= 40) return '#f0c070'
  if (progress > 0)   return baseColor
  return '#4e4a62'
}

export default function Tiles({ icon, name, sub, progress, badge, color, route }: TileProps) {
  const navigate  = useNavigate()
  const tileColor = getTileColor(progress, color)

  return (
    /* tile reste custom : CSS variables dynamiques --tc/--tg impossibles via Bulma */
    <button
      className="tile"
      style={{ '--tc': tileColor, '--tg': `${tileColor}20` } as React.CSSProperties}
      onClick={() => navigate(route)}
    >
      {/* tag Bulma pour le badge */}
      {badge && <span className="tag tile__badge">{badge}</span>}

      {/* tile__icon reste custom (taille+border fixes custom) */}
      <div >{icon}</div>

      <div>
        {/* is-size-7 + has-text-weight-semibold Bulma */}
        <div className="tile__name">{name}</div>
        <div className="tile__sub">{sub}</div>
      </div>

      {/* barre de progression : entierement custom, pas d'equivalent Bulma mince */}
      <div className="tile__bar-track">
        <div className="tile__bar-fill" style={{ width: `${progress}%` }} />
      </div>
    </button>
  )
}