// ============================================================
// HumeurPage.tsx  Journal d'humeur quotidien
// Picker 5 niveaux + note, calendrier 14 jours, modal de saisie
// Données stockées en localStorage sous "health_mood"
// ============================================================

import React, { useState, useEffect } from 'react';
import BackButton from '../../BackButton';
import { MoodEntry, MoodLevel } from '../../../types/health';
import { healthService } from '@/services/healthService';
import { useUserStore } from '@/store/userStore';
import '../../../styles/health.css';

const TODAY = new Date().toISOString().slice(0, 10);

const MOODS: { level: MoodLevel; emoji: string; label: string }[] = [
  { level: 1, emoji: '😞', label: 'Très bas' },
  { level: 2, emoji: '😕', label: 'Bas' },
  { level: 3, emoji: '😐', label: 'Neutre' },
  { level: 4, emoji: '🙂', label: 'Bien' },
  { level: 5, emoji: '😁', label: 'Excellent' },
];

function last14Days(): string[] {
  return Array.from({ length: 14 }, (_, i) => {
    const d = new Date();
    d.setDate(d.getDate() - (13 - i));
    return d.toISOString().slice(0, 10);
  });
}

const HumeurPage: React.FC = () => {
  const userId = useUserStore(s => s.profile?.id) ?? 0;
  const [entries, setEntries] = useState<MoodEntry[]>([]);

  const [modalOpen, setModalOpen] = useState(false);
  const [selectedLevel, setSelectedLevel] = useState<MoodLevel>(3);
  const [note, setNote] = useState('');

  useEffect(() => {
    void (async () => {
      try {
        setEntries(await healthService.getMoodEntries(userId));
      } catch {
        setEntries([]);
      }
    })();
  }, [userId]);

  const todayEntry = entries.find(e => e.date === TODAY);

  const openModal = () => {
    setSelectedLevel(todayEntry?.level ?? 3);
    setNote(todayEntry?.note ?? '');
    setModalOpen(true);
  };

  const handleSave = () => {
    const entry: MoodEntry = {
      id: todayEntry?.id ?? Date.now().toString(),
      date: TODAY,
      mood_type_id: selectedLevel,
      level: selectedLevel,
      note,
    };
    setEntries(prev =>
      todayEntry
        ? prev.map(e => e.id === todayEntry.id ? entry : e)
        : [...prev, entry]
    );
    void healthService.upsertMoodEntry(userId, entry);
    setModalOpen(false);
  };

  const days = last14Days();

  return (
    <section className="hero health-bg is-fullheight">
      <div className="hero-body" style={{ alignItems: 'flex-start', paddingTop: '2rem' }}>
        <div style={{ width: '100%' }}>
          <div className="page-header level is-mobile" style={{ marginBottom: '1rem' }}>
            <div className="level-left" style={{ gap: '.625rem' }}>
              <BackButton label="←" />
              <div>
                <p className="dash-sub is-size-7 has-text-weight-bold is-uppercase">Santé</p>
                <p className="dash-name">🎭 Humeur</p>
              </div>
            </div>
            <div className="level-right">
              <div className="level-item">
                <button className="button sober-add-btn"
                  style={{ background: 'var(--h-mood)', boxShadow: '0 4px 16px var(--h-mood-glow)' } as React.CSSProperties}
                  onClick={openModal}
                >+</button>
              </div>
            </div>
          </div>
          <hr />

          <div className="box health-box" style={{ marginTop: '1rem' }}>

            {/* Humeur du jour */}
            <div style={{ textAlign: 'center', padding: '.5rem 0' }}>
              {todayEntry ? (
                <>
                  <div style={{ fontSize: '3rem', lineHeight: 1 }}>
                    {MOODS.find(m => m.level === todayEntry.level)?.emoji}
                  </div>
                  <p style={{ color: 'var(--txt)', fontWeight: 600, marginTop: '.5rem' }}>
                    {MOODS.find(m => m.level === todayEntry.level)?.label}
                  </p>
                  {todayEntry.note && (
                    <p style={{ fontSize: '.8rem', color: 'var(--txt-muted)', marginTop: '.25rem', fontStyle: 'italic' }}>
                      "{todayEntry.note}"
                    </p>
                  )}
                  <div style={{ display: 'flex', gap: '.5rem', justifyContent: 'center', marginTop: '.5rem' }}>
                    <button
                      style={{ fontSize: '.75rem', color: 'var(--txt-faint)', background: 'none', border: 'none', cursor: 'pointer', textDecoration: 'underline' }}
                      onClick={openModal}
                    >Modifier</button>
                    <button
                      style={{ fontSize: '.75rem', color: 'var(--danger)', background: 'none', border: 'none', cursor: 'pointer', textDecoration: 'underline' }}
                      onClick={async () => {
                        if (!confirm("Supprimer l'humeur d'aujourd'hui ?")) return;
                        setEntries(prev => prev.filter(e => e.date !== TODAY));
                        try { await healthService.deleteMoodEntry(userId, TODAY); } catch {}
                      }}
                    >Supprimer</button>
                  </div>
                </>
              ) : (
                <div className="notification health-empty">
                  Aucune humeur enregistrée aujourd'hui.<br />
                  <button className="button health-btn-submit" style={{ marginTop: '.75rem', '--tc': 'var(--h-mood)' } as React.CSSProperties} onClick={openModal}>
                    Enregistrer mon humeur
                  </button>
                </div>
              )}
            </div>

            <hr style={{ margin: 0 }} />

            {/* Calendrier 14 jours */}
            <div>
              <p className="health-subtitle" style={{ marginBottom: '.625rem' }}>14 derniers jours</p>
              <div className="mood-calendar">
                {days.map(d => {
                  const e = entries.find(x => x.date === d);
                  const isToday = d === TODAY;
                  return (
                    <div key={d} className={`mood-day${isToday ? ' mood-day--today' : ''}`}>
                      <span className="mood-day-emoji">
                        {e ? MOODS.find(m => m.level === e.level)?.emoji : '·'}
                      </span>
                      <span className="mood-day-label">{d.slice(8)}</span>
                    </div>
                  );
                })}
              </div>
            </div>

          </div>
        </div>
      </div>

      {/* Modal de saisie */}
      <div className={`modal health-modal${modalOpen ? ' is-active' : ''}`}>
        <div className="modal-background" onClick={() => setModalOpen(false)} />
        <div className="modal-card health-modal-card">
          <header className="modal-card-head health-modal-head">
            <p className="modal-card-title health-modal-title">
              {todayEntry ? 'Modifier mon humeur' : 'Comment je me sens ?'}
            </p>
            <button className="delete" onClick={() => setModalOpen(false)} />
          </header>
          <section className="modal-card-body health-modal-body">
            {/* Picker humeur */}
            <div className="mood-picker">
              {MOODS.map(m => (
                <button
                  key={m.level}
                  className={`button mood-btn${selectedLevel === m.level ? ' is-active' : ''}`}
                  title={m.label}
                  onClick={() => setSelectedLevel(m.level)}
                >
                  {m.emoji}
                </button>
              ))}
            </div>
            <p style={{ textAlign: 'center', color: 'var(--txt-muted)', fontSize: '.8rem' }}>
              {MOODS.find(m => m.level === selectedLevel)?.label}
            </p>
            <div className="field">
              <label className="label health-label">Note (optionnel)</label>
              <div className="control">
                <textarea
                  className="textarea health-input"
                  rows={2}
                  placeholder="Ce qui influence mon humeur..."
                  value={note}
                  onChange={e => setNote(e.target.value)}
                />
              </div>
            </div>
          </section>
          <footer className="modal-card-foot health-modal-foot" style={{ justifyContent: 'flex-end', gap: '.5rem' }}>
            <button className="button health-btn-cancel" onClick={() => setModalOpen(false)}>Annuler</button>
            <button className="button health-btn-submit" onClick={handleSave}
              style={{ '--tc': 'var(--h-mood)', '--tg': 'var(--h-mood-glow)' } as React.CSSProperties}>
              Enregistrer
            </button>
          </footer>
        </div>
      </div>
    </section>
  );
};

export default HumeurPage;