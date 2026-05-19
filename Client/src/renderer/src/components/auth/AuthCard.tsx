import { useState } from 'react'
import LoginForm    from './LoginForm'
import RegisterForm from './RegisterForm'

export default function AuthCard() {
  const [tab, setTab] = useState<'login' | 'register'>('login')

  return (
    <section className="hero is-fullheight auth-bg">
      <div className="hero-body">
        <div className="container">

          <div className="box auth-box">

            {/* Logo */}
            <div className="has-text-centered mb-4">
              <span className="auth-logo__img">🌿</span>
            </div>

            {/* Tabs */}
            <div className="tabs is-toggle is-fullwidth mb-4" role="tablist">
              <ul>
                <li className={tab === 'login' ? 'is-active' : ''}>
                  <a role="tab" aria-selected={tab === 'login'}
                     onClick={() => setTab('login')}>
                    Connexion
                  </a>
                </li>
                <li className={tab === 'register' ? 'is-active' : ''}>
                  <a role="tab" aria-selected={tab === 'register'}
                     onClick={() => setTab('register')}>
                    Inscription
                  </a>
                </li>
              </ul>
            </div>

            {/* Heading */}
            <div className="mb-4">
              {tab === 'login' ? (
                <>
                  <h1 className="title is-5">👋 Bon retour</h1>
                  <p className="subtitle is-6">Connectez-vous pour continuer.</p>
                </>
              ) : (
                <>
                  <h1 className="title is-5">✨ Creer un compte</h1>
                  <p className="subtitle is-6">Commence a suivre ta vie des aujourd'hui.</p>
                </>
              )}
            </div>

            {/* Formulaire */}
            {tab === 'login' ? <LoginForm /> : <RegisterForm />}

            {/* Divider */}
            <div className="auth-divider my-4">ou</div>

            {/* Switch */}
            <p className="has-text-centered is-size-7 auth-switch-text">
              {tab === 'login' ? (
                <>Pas encore de compte ?{' '}
                  <a onClick={() => setTab('register')}>S'inscrire</a>
                </>
              ) : (
                <>Deja un compte ?{' '}
                  <a onClick={() => setTab('login')}>Se connecter</a>
                </>
              )}
            </p>

          </div>
        </div>
      </div>
    </section>
  )
}
