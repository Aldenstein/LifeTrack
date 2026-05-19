// ============================================================
// HealthHome.tsx  Page d'accueil du module Santé
// Grille de 9 tuiles, une par sous-module
// Même structure que la page Home existante (hero + box + grille responsive)
// ============================================================

import React from 'react';
import { useNavigate } from 'react-router-dom';
import BackButton from '../BackButton';
import HealthTuile from './HealthTuile';
import '../../styles/health.css';

// Définition des 9 sous-modules avec leurs métadonnées visuelles
const MODULES = [
  {
    key: 'sommeil',
    emoji: '😴',
    name: 'Sommeil',
    sub: 'Durée & qualité de nuit',
    color: '#5b7bab',
    glow: 'rgba(91,123,171,.35)',
    path: '/health/sommeil',
  },
  {
    key: 'hydratation',
    emoji: '💧',
    name: 'Hydratation',
    sub: 'Suivi de la consommation d\'eau',
    color: '#4fc3d4',
    glow: 'rgba(79,195,212,.35)',
    path: '/health/hydratation',
  },
  {
    key: 'humeur',
    emoji: '🎭',
    name: 'Humeur',
    sub: 'Journal émotionnel quotidien',
    color: '#d4a853',
    glow: 'rgba(212,168,83,.35)',
    path: '/health/humeur',
  },
  {
    key: 'ponderaux',
    emoji: '⚖️',
    name: 'Calculs pondéraux',
    sub: 'IMC, MB, NEAT et poids idéal',
    color: '#6aab7a',
    glow: 'rgba(106,171,122,.35)',
    path: '/health/ponderaux',
  },
  {
    key: 'alcoolemie',
    emoji: '🍷',
    name: 'Alcoolémie',
    sub: 'Estimation Widmark',
    color: '#c4607a',
    glow: 'rgba(196,96,122,.35)',
    path: '/health/alcoolemie',
  },
  {
    key: 'sobriete',
    emoji: '🌱',
    name: 'Sobriété',
    sub: 'Cartes & compteurs de sobriété',
    color: '#5cad7a',
    glow: 'rgba(92,173,122,.35)',
    path: '/health/sobriete',
  },
  {
    key: 'coherence',
    emoji: '🫁',
    name: 'Cohérence cardiaque',
    sub: 'Respiration guidée',
    color: '#9d93e0',
    glow: 'rgba(157,147,224,.35)',
    path: '/health/coherence',
  },
  {
    key: 'repas',
    emoji: '🍽️',
    name: 'Repas',
    sub: 'Calories consommées',
    color: '#d4a853',
    glow: 'rgba(212,168,83,.35)',
    path: '/health/repas',
  },
  {
    key: 'sport',
    emoji: '🏋️',
    name: 'Sport',
    sub: 'Calories brûlées',
    color: '#4fc3d4',
    glow: 'rgba(79,195,212,.35)',
    path: '/health/sport',
  },
] as const;

const HealthHome: React.FC = () => {
  const navigate = useNavigate();

  return (
    <section className="hero health-bg is-fullheight">
      <div className="hero-body" style={{ alignItems: 'flex-start', paddingTop: '2rem' }}>
        <div style={{ width: '100%' }}>

          {/* En-tête de page  BackButton + titre */}
          <div className="page-header level is-mobile" style={{ marginBottom: '1rem' }}>
            <div className="level-left" style={{ gap: '.625rem' }}>
              <BackButton label="←" />
              <div>
                <p className="dash-sub is-size-7 has-text-weight-bold is-uppercase">Module</p>
                <p className="dash-name">Santé</p>
              </div>
            </div>
          </div>

          <hr />

          {/* Box principale avec la grille de tuiles */}
          <div className="box health-box health-home-box" style={{ marginTop: '1rem' }}>
            <div className="dash-grid">
              {MODULES.map((mod) => (
                <div className="health-grid-item" key={mod.key}>
                  <HealthTuile
                    emoji={mod.emoji}
                    name={mod.name}
                    sub={mod.sub}
                    color={mod.color}
                    glow={mod.glow}
                    onClick={() => navigate(mod.path)}
                  />
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default HealthHome;
