import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '@/hooks/useAuth'

export default function RegisterForm() {
  const navigate = useNavigate()
  const { register, loading, error } = useAuth()

  const [email, setEmail] = useState('')
  // Correction : "password" -> "passphrase" (terme exact de l'API LifeTrack)
  const [passphrase, setPassphrase] = useState('')
  const [showPwd, setShowPwd] = useState(false)
  const [localError, setLocalError] = useState('')

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    setLocalError('')
    // Correction : suppression du champ "name/username" — absent de l'API
    if (!email || !passphrase) { setLocalError('Remplis tous les champs.'); return }
    if (passphrase.length < 8) { setLocalError('Passphrase trop courte (min. 8 caracteres).'); return }
    // Correction : { username, email, password } -> { email, passphrase }
    const ok = await register({ email, passphrase })
    if (ok) navigate('/')
  }

  return (
    <form onSubmit={handleSubmit}>

      {/* Email */}
      <div className="field">
        <label className="label">Adresse email</label>
        <div className="control">
          <input className="input" type="email" placeholder="votre@email.com"
            value={email} onChange={(e) => setEmail(e.target.value)} />
        </div>
      </div>

      {/* Passphrase */}
      <div className="field">
        {/* Correction : label "Mot de passe" -> "Passphrase" */}
        <label className="label">Passphrase</label>
        {/* Avertissement : la passphrase chiffre les donnees Zero-Knowledge */}
        <p className="help mb-2 has-text-warning-dark">
          ⚠️ Ta passphrase chiffre tes donnees. Si tu la perds, tes donnees ne pourront
          plus jamais etre recuperees.
        </p>
        <div className="field has-addons mb-0">
          <div className="control is-expanded">
            <input className="input psw" type={showPwd ? 'text' : 'password'}
              placeholder="••••••••" value={passphrase}
              onChange={(e) => setPassphrase(e.target.value)} />
          </div>
          <div className="control">
            <button type="button" className="button auth-eye-btn"
              onClick={() => setShowPwd(!showPwd)}>
              {showPwd ? '🙈' : '👁️'}
            </button>
          </div>
        </div>
      </div>

      {/* Erreur */}
      {(localError || error) && <p className="help is-danger mb-3">{localError || error}</p>}

      {/* Submit */}
      <button className="button is-fullwidth auth-submit mt-2" type="submit" disabled={loading}>
        {loading ? 'Creation...' : 'Creer mon compte'}
      </button>

    </form>
  )
}
