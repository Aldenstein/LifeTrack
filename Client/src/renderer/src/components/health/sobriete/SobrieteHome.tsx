// ============================================================
// SobrieteHome.tsx  Page principale du module Sobriété
// Liste des cartes de sobriété + création via modal
// Données stockées en localStorage sous "health_sober"
// ============================================================

import React, { useState, useEffect } from 'react';
import BackButton from '../../BackButton';
import SobrieteCardComp from './SobrieteCard';
import { SobrieteCard } from '../../../types/health';
import { healthService } from '@/services/healthService';
import { useUserStore } from '@/store/userStore';
import '../../../styles/health.css';

const TODAY = new Date().toISOString().slice(0, 10);

// Couleurs et icônes proposés à la création
const COLORS = ['#5cad7a', '#9d93e0', '#4fc3d4', '#d4a853', '#c4607a', '#5b7bab'];
const ICONS  = ['🚬', '🍺', '🍷', '🎰', '🍭', '📱', '💊', '☕', '🍫'];

const SobrieteHome: React.FC = () => {
  const userId = useUserStore(s => s.profile?.id) ?? 0;
  const [cards, setCards] = useState<SobrieteCard[]>([]);

  const [modalOpen, setModalOpen] = useState(false);
  const [substance, setSubstance] = useState('');
  const [color, setColor]         = useState(COLORS[0]);
  const [icon, setIcon]           = useState(ICONS[0]);

  useEffect(() => {
    void (async () => {
      try {
        setCards(await healthService.getSobrieteCards(userId));
      } catch {
        setCards([]);
      }
    })();
  }, [userId]);

  const handleCreate = async () => {
    if (!substance.trim()) return;
    const newCard: SobrieteCard = {
      id: Date.now().toString(),
      substance: substance.trim(),
      startDate: TODAY,
      color, icon,
    };
    try {
      const created = await healthService.createSobrieteCard(userId, newCard);
      setCards(prev => [...prev, created]);
    } catch {
      return;
    }
    setSubstance(''); setColor(COLORS[0]); setIcon(ICONS[0]);
    setModalOpen(false);
  };

  // Rechute : remet la date de départ à aujourd'hui
  const handleRelapse = async (id: string) => {
    const current = cards.find(c => c.id === id);
    if (!current) return;
    const updated = { ...current, startDate: TODAY };
    try {
      await healthService.updateSobrieteCard(userId, id, updated);
      setCards(prev => prev.map(c => c.id === id ? updated : c));
    } catch {
      return;
    }
  };

  const handleDelete = async (id: string) => {
    try {
      await healthService.deleteSobrieteCard(userId, id);
      setCards(prev => prev.filter(c => c.id !== id));
    } catch {
      return;
    }
  };

  // Tri par ancienneté décroissante (le plus ancien en premier)
  const sorted = [...cards].sort((a, b) => a.startDate.localeCompare(b.startDate));

  return (
    <section className="hero health-bg is-fullheight">
      <div className="hero-body" style={{ alignItems: 'flex-start', paddingTop: '2rem' }}>
        <div style={{ width: '100%' }}>
          <div className="page-header level is-mobile" style={{ marginBottom: '1rem' }}>
            <div className="level-left" style={{ gap: '.625rem' }}>
              <BackButton label="←" />
              <div>
                <p className="dash-sub is-size-7 has-text-weight-bold is-uppercase">Santé</p>
                <p className="dash-name">🌱 Sobriété</p>
              </div>
            </div>
            <div className="level-right">
              <div className="level-item">
                <button className="button sober-add-btn" onClick={() => setModalOpen(true)}>+</button>
              </div>
            </div>
          </div>
          <hr />

          <div className="box health-box" style={{ marginTop: '1rem' }}>
            {sorted.length === 0 ? (
              <div className="notification health-empty">
                Aucune carte de sobriété.<br />
                Créez-en une avec le bouton +
              </div>
            ) : (
              <div style={{ display: 'flex', flexDirection: 'column', gap: '.75rem' }}>
                {sorted.map(card => (
                  <SobrieteCardComp
                    key={card.id}
                    card={card}
                    onRelapse={handleRelapse}
                    onDelete={handleDelete}
                  />
                ))}
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Modal création */}
      <div className={`modal health-modal${modalOpen ? ' is-active' : ''}`}>
        <div className="modal-background" onClick={() => setModalOpen(false)} />
        <div className="modal-card health-modal-card">
          <header className="modal-card-head health-modal-head">
            <p className="modal-card-title health-modal-title">Nouvelle carte de sobriété</p>
            <button className="delete" onClick={() => setModalOpen(false)} />
          </header>
          <section className="modal-card-body health-modal-body">
            <div className="field">
              <label className="label health-label">Substance</label>
              <input className="input health-input" type="text"
                placeholder="Alcool, Tabac, Cannabis…"
                value={substance} onChange={e => setSubstance(e.target.value)} />
            </div>

            <div className="field">
              <label className="label health-label">Icône</label>
              <div style={{ display: 'flex', gap: '.375rem', flexWrap: 'wrap' }}>
                {ICONS.map(ic => (
                  <button key={ic}
                    className={`button habit-icon-btn${icon === ic ? ' is-active' : ''}`}
                    onClick={() => setIcon(ic)}
                  >{ic}</button>
                ))}
              </div>
            </div>

            <div className="field">
              <label className="label health-label">Couleur</label>
              <div style={{ display: 'flex', gap: '.5rem' }}>
                {COLORS.map(c => (
                  <button key={c}
                    className="habit-color-btn"
                    style={{
                      background: c,
                      width: 28, height: 28, borderRadius: '50%', border: 'none', cursor: 'pointer',
                      outline: color === c ? `3px solid ${c}` : 'none',
                      outlineOffset: 2,
                      transition: 'transform 200ms',
                    }}
                    onClick={() => setColor(c)}
                  />
                ))}
              </div>
            </div>
          </section>
          <footer className="modal-card-foot health-modal-foot" style={{ justifyContent: 'flex-end', gap: '.5rem' }}>
            <button className="button health-btn-cancel" onClick={() => setModalOpen(false)}>Annuler</button>
            <button className="button health-btn-submit" onClick={handleCreate}
              disabled={!substance.trim()}
              style={{ '--tc': 'var(--h-sober)', '--tg': 'var(--h-sober-glow)' } as React.CSSProperties}>
              Créer
            </button>
          </footer>
        </div>
      </div>
    </section>
  );
};

export default SobrieteHome;