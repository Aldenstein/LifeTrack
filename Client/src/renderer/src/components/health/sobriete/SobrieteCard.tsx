// ============================================================
// SobrieteCard.tsx  Carte individuelle de sobriété
// Affiche : substance, jours de sobriété, palier, bouton rechute
// La rechute demande une confirmation inline avant de s'exécuter
// ============================================================

import React, { useState } from 'react';
import { SobrieteCard as SobrieteCardType } from '../../../types/health';

// Paliers de sobriété avec emoji et nombre de jours
const MILESTONES = [
  { days: 365, emoji: '🏆', label: '1 an' },
  { days: 90,  emoji: '🌳', label: '90 jours' },
  { days: 30,  emoji: '🌿', label: '30 jours' },
  { days: 7,   emoji: '🌱', label: '1 semaine' },
];

interface Props {
  card: SobrieteCardType;
  onRelapse: (id: string) => void;
  onDelete: (id: string) => void;
}

const SobrieteCardComp: React.FC<Props> = ({ card, onRelapse, onDelete }) => {
  const [confirming, setConfirming] = useState(false);

  // Calcule le nombre de jours depuis startDate
  const start = new Date(card.startDate);
  const now   = new Date();
  const days  = Math.floor((now.getTime() - start.getTime()) / (1000 * 60 * 60 * 24));

  // Palier actuel atteint
  const milestone = MILESTONES.find(m => days >= m.days);

  // Prochain palier
  const nextMilestone = [...MILESTONES].reverse().find(m => m.days > days);
  const nextDays = nextMilestone ? nextMilestone.days - days : null;
  const pct = nextMilestone
    ? Math.min(((days / nextMilestone.days) * 100), 100)
    : 100;

  const handleRelapseClick = () => {
    if (confirming) {
      onRelapse(card.id);
      setConfirming(false);
    } else {
      setConfirming(true);
      // Auto-annulation si l'utilisateur ne confirme pas en 3s
      setTimeout(() => setConfirming(false), 3000);
    }
  };

  return (
    <div className="sober-card" style={{ '--sc': card.color } as React.CSSProperties}>

      {/* En-tête : icône + nom */}
      <div className="sober-card-head">
        <span className="sober-card-icon">{card.icon}</span>
        <span className="sober-card-name">{card.substance}</span>

        {/* Bouton suppression discret */}
        <button
          className="delete is-small"
          style={{ marginLeft: 'auto' }}
          onClick={() => onDelete(card.id)}
          title="Supprimer cette carte"
        />
      </div>

      {/* Compteur de jours */}
      <div>
        <div className="sober-card-days">{days}</div>
        <div className="sober-card-days-label">jours de sobriété</div>
      </div>

      {/* Badge palier atteint */}
      {milestone && (
        <span className="sober-card-milestone">
          {milestone.emoji} {milestone.label} atteint !
        </span>
      )}

      {/* Barre vers le prochain palier */}
      {nextMilestone && (
        <div>
          <div className="health-progress-track" style={{ '--tc': card.color } as React.CSSProperties}>
            <div className="health-progress-fill" style={{ width: `${pct}%`, background: card.color, boxShadow: `0 0 8px ${card.color}60` }} />
          </div>
          <p style={{ fontSize: '.68rem', color: 'var(--txt-faint)', marginTop: '.25rem' }}>
            {nextDays}j avant {nextMilestone.emoji} {nextMilestone.label}
          </p>
        </div>
      )}

      {/* Bouton rechute avec confirmation inline */}
      <button
        className={`button sober-relapse-btn${confirming ? ' is-confirming' : ''}`}
        onClick={handleRelapseClick}
        style={{ alignSelf: 'flex-start' }}
      >
        {confirming ? '⚠️ Confirmer la rechute' : 'Rechute'}
      </button>

    </div>
  );
};

export default SobrieteCardComp;