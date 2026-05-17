// ============================================================
// HealthTuile.tsx  Tuile cliquable réutilisable du module Santé
// Reprend la structure visuelle des tiles de Home (home.css)
// Props : emoji, name, sub (description), color (accent hex),
//         glow (rgba pour le glow), badge (texte optionnel), onClick
// ============================================================

import React from 'react';

interface HealthTuileProps {
  emoji: string;
  name: string;
  sub: string;
  color: string;        // ex: "#5b7bab"
  glow: string;         // ex: "rgba(91,123,171,.35)"
  badge?: string;       // ex: "7h30" ou "80%"  affiché en haut à droite
  onClick: () => void;
}

const HealthTuile: React.FC<HealthTuileProps> = ({
  emoji, name, sub, color, glow, badge, onClick
}) => {
  return (
    // Bouton tuile  couleur injectée via CSS custom properties
    <button
      className="health-tile"
      style={{ '--tc': color, '--tg': glow } as React.CSSProperties}
      onClick={onClick}
    >
      {/* Badge optionnel en haut à droite (ex: valeur du jour) */}
      {badge && (
        <span className="tag health-tile-badge">{badge}</span>
      )}

      {/* Icône emoji dans un carré arrondi */}
      <div className="health-tile-icon">{emoji}</div>

      {/* Nom du sous-module */}
      <span className="health-tile-name">{name}</span>

      {/* Description courte */}
      <span className="health-tile-sub">{sub}</span>
    </button>
  );
};

export default HealthTuile;
