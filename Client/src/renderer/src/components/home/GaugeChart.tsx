interface GaugeChartProps {
  value: number
  size?: number
  label?: string
}

function getGaugeColor(value: number): string {
  if (value >= 75) return '#7ec8a4'
  if (value >= 40) return '#f0c070'
  return '#e07c9d'
}

export default function GaugeChart({ value, size = 240, label = 'Accompli' }: GaugeChartProps) {
  const color  = getGaugeColor(value)
  const r      = 90
  const cx     = size / 2
  const cy     = size / 2
  const stroke = 12

  const toRad = (deg: number) => (deg * Math.PI) / 180
  const getXY = (deg: number) => ({
    x: cx + r * Math.cos(toRad(deg)),
    y: cy + r * Math.sin(toRad(deg)),
  })

  const tStart  = getXY(180)
  const tEnd    = getXY(360)
  const fillDeg = 180 + (value / 100) * 180
  const fEnd    = getXY(fillDeg)

  const trackPath = `M ${tStart.x} ${tStart.y} A ${r} ${r} 0 1 1 ${tEnd.x} ${tEnd.y}`
  const fillPath  = value > 0
    ? `M ${tStart.x} ${tStart.y} A ${r} ${r} 0 ${fillDeg - 180 >= 180 ? 1 : 0} 1 ${fEnd.x} ${fEnd.y}`
    : ''

  const h = size / 2 + stroke

  return (
    /* gauge-wrap reste custom : positionnement relatif pour le texte centré */
    <div className="gauge-wrap" style={{ width: '100%' }}>
      <svg width="100%" height="auto" viewBox={`0 0 ${size} ${h}`}>
        <path d={trackPath} fill="none" stroke="rgba(255,255,255,.07)"
          strokeWidth={stroke} strokeLinecap="round" />
        {value > 0 && (
          <path d={fillPath} fill="none" stroke={color}
            strokeWidth={stroke} strokeLinecap="round"
            style={{ filter: `drop-shadow(0 0 3px ${color}90)`, transition: 'all 1s cubic-bezier(.16,1,.3,1)' }} />
        )}
        {value > 0 && (
          <circle cx={fEnd.x} cy={fEnd.y} r={5} fill={color}
            style={{ filter: `drop-shadow(0 0 6px ${color})` }} />
        )}
      </svg>

      {/* gauge-center reste custom : position absolute sur SVG demi-cercle */}
      <div className="gauge-center">
        <span className="gauge-center__val" style={{ color }}>{value}%</span>
        {/* is-uppercase is-size-7 Bulma pour le label */}
        <span className="is-size-7 has-text-weight-bold gauge-lbl">{label}</span>
      </div>
    </div>
  )
}