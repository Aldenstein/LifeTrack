import { HashRouter, Routes, Route, Navigate } from 'react-router-dom'
import { useBootstrap }  from '@/hooks/useBootstrap'
import { useUserStore }  from '@/store/userStore'
import AuthPage          from '@/pages/AuthPage'
import HomePage          from '@/pages/HomePage'
import HabitsPage        from '@/pages/HabitsPage'
import TodoPage          from '@/pages/TodoPage'
import FinancePage       from '@/pages/FinancePage'
import SommeilPage      from './components/health/sommeil/SommeilPage';
import HealthHome       from './components/health/HealthHome';
import HydratationPage  from './components/health/hydratation/HydratationPage';
import HumeurPage       from './components/health/humeur/HumeurPage';
import PonderauxPage    from './components/health/ponderaux/PonderauxPage';
import AlcoolemiePage   from './components/health/alcoolemie/AlcoolemiePage';
import SobrieteHome     from './components/health/sobriete/SobrieteHome';
import CoherencePage    from './components/health/coherence/CoherencePage';
import RepasPage        from './components/health/repas/RepasPage';
import SportPage        from './components/health/sport/SportPage';

function PrivateRoute({ children }: { children: React.ReactNode }) {
  const token = useUserStore(s => s.token)
  return token ? <>{children}</> : <Navigate to="/auth" replace />
}

export default function App() {
  const { ready } = useBootstrap()

  if (!ready) return (
    <div className="hero is-fullheight">
      <div className="hero-body has-text-centered">
        <div>
          <p className="title">GrattFinance</p>
          <p className="subtitle">Chargement...</p>
        </div>
      </div>
    </div>
  )

  return (
    <HashRouter>
      <Routes>

        {/* Route publique */}
        <Route path="/auth" element={<AuthPage />} />

        {/* Routes protégées */}
        <Route path="/" element={
          <PrivateRoute><HomePage /></PrivateRoute>
        }/>
        <Route path="/habits" element={
          <PrivateRoute><HabitsPage /></PrivateRoute>
        }/>
        <Route path="/todos" element={
          <PrivateRoute><TodoPage /></PrivateRoute>
        }/>
        <Route path="/finance" element={
          <PrivateRoute><FinancePage /></PrivateRoute>
        }/>
        <Route path="/health"            element={
          <PrivateRoute><HealthHome /></PrivateRoute>
        }/>
        <Route path="/health/hydratation" element={<PrivateRoute><HydratationPage /></PrivateRoute>} />
        <Route path="/health/humeur"      element={<PrivateRoute><HumeurPage /></PrivateRoute>} />
        <Route path="/health/ponderaux"   element={<PrivateRoute><PonderauxPage /></PrivateRoute>} />
        <Route path="/health/alcoolemie"  element={<PrivateRoute><AlcoolemiePage /></PrivateRoute>} />
        <Route path="/health/sobriete"    element={<PrivateRoute><SobrieteHome /></PrivateRoute>} />
        <Route path="/health/sommeil"     element={<PrivateRoute><SommeilPage /></PrivateRoute>} />
        <Route path="/health/coherence"   element={<PrivateRoute><CoherencePage /></PrivateRoute>} />
        <Route path="/health/repas"       element={<PrivateRoute><RepasPage /></PrivateRoute>} />
        <Route path="/health/sport"       element={<PrivateRoute><SportPage /></PrivateRoute>} />

        {/* Fallback */}
        <Route path="*" element={<Navigate to="/auth" replace />} />

      </Routes>
    </HashRouter>
  )
}