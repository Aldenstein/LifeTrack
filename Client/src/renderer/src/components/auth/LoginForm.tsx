import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '@/hooks/useAuth'

export default function LoginForm() {
  const navigate = useNavigate()
  const { login, loading, error } = useAuth()

  const [email, setEmail] = useState('')
  const [password, setPassword] = useState('')
  const [showPwd, setShowPwd] = useState(false)
  const [localError, setLocalError] = useState('')

  async function handleSubmit(e: React.FormEvent) {
    e.preventDefault()
    setLocalError('')
    if (!email || !password) { setLocalError('Remplis tous les champs.'); return }
    const ok = await login({ email, password })
    if (ok) navigate('/')
  }

  return (
    <form onSubmit={handleSubmit}>

      {/* Email */}
      <div className="field">
        <label className="label">Adresse email</label>
        <div className="control">
          <input
            className={`input${error ? ' is-danger' : ''}`}
            type="email"
            placeholder="votre@email.com"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />
        </div>
      </div>

      {/* Mot de passe  has-addons colle le bouton oeil à l'input */}
      <div className="field">
        <label className="label">Mot de passe</label>
        <div className="field has-addons mb-0">
          <div className="control is-expanded psw">
            <input
              className={`input psw${error ? ' is-danger' : ''}`}
              type={showPwd ? 'text' : 'password'}
              placeholder="••••••••"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
            />
          </div>
          {/* .control > .button colle naturellement à l'input avec has-addons */}
          <div className="control">
            <button type="button" className="button auth-eye-btn"
              onClick={() => setShowPwd(!showPwd)}>
              {showPwd ? '🙈' : '👁️'}
            </button>
          </div>
        </div>
      </div>

      {/* Erreur */}
      {(localError || error) && <p className="help is-danger mb-2">{localError || error}</p>}

      {/* Mot de passe oublié  .has-text-right Bulma */}
      <div className="has-text-right mb-4">
        <a className="is-size-7 auth-link">Mot de passe oublié ?</a>
      </div>

      {/* Submit  .button.is-fullwidth Bulma */}
      <button className="button is-fullwidth auth-submit" type="submit" disabled={loading}>
        {loading ? 'Connexion…' : 'Se connecter'}
      </button>

    </form>
  )
}