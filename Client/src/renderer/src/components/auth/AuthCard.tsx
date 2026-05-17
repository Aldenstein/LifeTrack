import { useState } from 'react'
import LoginForm    from './LoginForm'
import RegisterForm from './RegisterForm'

export default function AuthCard() {
  const [tab, setTab] = useState<'login' | 'register'>('login')

  return (
    /* hero + hero-body = pleine hauteur centrée  remplace auth-page + auth-card */
    <section className="hero is-fullheight auth-bg">
      <div className="hero-body">
        <div className="container">

          {/* box = card Bulma légère  remplace auth-card */}
          <div className="box auth-box">

            {/* Logo  pas d'équivalent Bulma, 3 lignes custom suffisent */}
            <div className="has-text-centered mb-4">
              <span className="auth-logo__img">🌿</span>
            </div>

            {/* Tabs Bulma is-toggle is-fullwidth  remplace auth-tabs pill */}
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

            {/* Heading  .title + .subtitle Bulma natifs */}
            <div className="mb-4">
              {tab === 'login' ? (
                <>
                  <h1 className="title is-5">{String.fromCodePoint(0x1F44B)} Bon retour</h1>
                  <p className="subtitle is-6">Connectez-vous pour continuer.</p>
                </>
              ) : (
                <>
                  <h1 className="title is-5">{String.fromCodePoint(0x2728)} Créer un compte</h1>
                  <p className="subtitle is-6">Commence à suivre ta vie dès aujourd'hui.</p>
                </>
              )}
            </div>

            {/* Formulaire */}
            {tab === 'login' ? <LoginForm /> : <RegisterForm />}

            {/* Divider  une seule ligne custom */}
            <div className="auth-divider my-4">ou</div>

            {/* Switch  .has-text-centered + helpers Bulma */}
            <p className="has-text-centered is-size-7 auth-switch-text">
              {tab === 'login' ? (
                <>Pas encore de compte ?{' '}
                  <a onClick={() => setTab('register')}>S'inscrire</a>
                </>
              ) : (
                <>Déjà un compte ?{' '}
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