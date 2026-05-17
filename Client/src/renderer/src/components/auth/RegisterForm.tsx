import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '@/hooks/useAuth'

export default function RegisterForm() {
  const navigate = useNavigate()
  const { register, loading, error } = useAuth()

  const [name, setName] = useState('')
  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [showPwd, setShowPwd] = useState(false)
  const [localError, setLocalError] = useState('')

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    setLocalError('')
    if (!name || !email || !password) { setLocalError('Remplis tous les champs.'); return }
    if (password.length < 6) { setLocalError('Mot de passe trop court (min. 6 caractères).'); return }
    const ok = await register({ username: name, email, password })
    if (ok) navigate('/')
  }

  return (
    <form onSubmit={handleSubmit}>

      {/* Prénom */}
      <div className="field">
        <label className="label">Prénom</label>
        <div className="control">
          <input className="input" type="text" placeholder="Votre prénom"
            value={name} onChange={(e) => setName(e.target.value)} />
        </div>
      </div>

      {/* Email */}
      <div className="field">
        <label className="label">Adresse email</label>
        <div className="control">
          <input className="input" type="email" placeholder="votre@email.com"
            value={email} onChange={(e) => setEmail(e.target.value)} />
        </div>
      </div>

      {/* Mot de passe */}
      <div className="field">
        <label className="label">Mot de passe</label>
        <div className="field has-addons mb-0">
          <div className="control is-expanded">
            <input className="input psw" type={showPwd ? 'text' : 'password'}
              placeholder="••••••••" value={password}
              onChange={(e) => setPassword(e.target.value)} />
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
        {loading ? 'Création…' : 'Créer mon compte'}
      </button>

    </form>
  )
}