import { useNavigate } from 'react-router-dom'
import '@/styles/back-button.css'

interface BackButtonProps {
  to?: string   // optionnel  par défaut revient en arrière (-1 dans l'historique)
  label?: string
  onClick?: () => void
}

export default function BackButton({ to, label = 'Retour', onClick }: BackButtonProps) {
  const navigate = useNavigate()

  // Navigue vers une route spécifique ou exécute le handler fourni
  function handle() {
    if (onClick) return onClick()
    if (to) return navigate(to)
    return navigate(-1)
  }

  return (
    <button
      className="back-btn"
      onClick={handle}
    >
       {label}
    </button>
  )
}