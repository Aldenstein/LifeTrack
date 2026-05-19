// ============================================================
// SportPage.tsx  Suivi des séances de sport et calories brûlées
// Saisie simple : activité, durée, poids, note
// Calcul basé sur une estimation MET
// Données stockées en localStorage sous "health_sport"
// ============================================================

import React, { useEffect, useMemo, useState } from 'react';
import BackButton from '../../BackButton';
import { SportSessionEntry, SportType } from '../../../types/health';
import { healthService } from '@/services/healthService';
import { useUserStore } from '@/store/userStore';
import '../../../styles/health.css';

const TODAY = new Date().toISOString().slice(0, 10);

const SPORT_TYPES: { value: SportType; label: string; emoji: string; met: number }[] = [
  { value: 'running', label: 'Course', emoji: '🏃', met: 8.3 },
  { value: 'cycling', label: 'Vélo', emoji: '🚴', met: 7.0 },
  { value: 'walking', label: 'Marche', emoji: '🚶', met: 3.5 },
  { value: 'musculation', label: 'Musculation', emoji: '🏋️', met: 6.0 },
  { value: 'hiit', label: 'HIIT', emoji: '🔥', met: 9.0 },
  { value: 'other', label: 'Autre', emoji: '⚡', met: 5.0 },
];

function calcCalories(met: number, weightKg: number, durationMin: number): number {
  return Math.round(met * weightKg * durationMin * 0.0175);
}

const SportPage: React.FC = () => {
  const userId = useUserStore(s => s.profile?.id) ?? 0;
  const [sessions, setSessions] = useState<SportSessionEntry[]>([]);
  const [editingSessionId, setEditingSessionId] = useState<string | null>(null);

  const [sportType, setSportType] = useState<SportType>('running');
  const [durationMin, setDurationMin] = useState(30);
  const [weightKg, setWeightKg] = useState(70);
  const [note, setNote] = useState('');

  useEffect(() => {
    void (async () => {
      try {
        setSessions(await healthService.getSportSessions(userId));
      } catch {
        setSessions([]);
      }
    })();
  }, [userId]);

  const selectedSport = SPORT_TYPES.find(s => s.value === sportType) ?? SPORT_TYPES[0];
  const caloriesPreview = calcCalories(selectedSport.met, weightKg, durationMin);

  const todaySessions = useMemo(
    () => sessions.filter(s => s.date === TODAY).sort((a, b) => b.id.localeCompare(a.id)),
    [sessions]
  );
  const todayBurned = todaySessions.reduce((sum, s) => sum + s.calories, 0);
  const weekBurned = useMemo(() => {
    const days = Array.from({ length: 7 }, (_, i) => {
      const d = new Date();
      d.setDate(d.getDate() - i);
      return d.toISOString().slice(0, 10);
    });
    return sessions.filter(s => days.includes(s.date)).reduce((sum, s) => sum + s.calories, 0);
  }, [sessions]);

  const resetForm = () => {
    setEditingSessionId(null);
    setSportType('running');
    setDurationMin(30);
    setWeightKg(70);
    setNote('');
  };

  const handleSave = async () => {
    if (durationMin <= 0 || weightKg <= 0) return;

    const entry: SportSessionEntry = {
      id: editingSessionId ?? Date.now().toString(),
      date: TODAY,
      time: new Date().toTimeString().slice(0, 8),
      type_id: 1,
      sportType,
      duration: durationMin,
      calories: calcCalories(selectedSport.met, weightKg, durationMin),
      intensity: 'modéré',
      note: note.trim() || undefined,
    };

    try {
      if (editingSessionId) {
        const updated = await healthService.updateSportSession(userId, editingSessionId, entry);
        setSessions(prev => prev.map(s => (s.id === editingSessionId ? updated : s)));
      } else {
        const created = await healthService.createSportSession(userId, entry);
        setSessions(prev => [created, ...prev]);
      }
    } catch {
      return;
    }
    resetForm();
  };

  const handleEdit = (session: SportSessionEntry) => {
    setEditingSessionId(session.id);
    setSportType(session.sportType ?? 'running');
    setDurationMin(session.duration);
    setNote(session.note ?? '');
  };

  const handleCancelEdit = () => {
    resetForm();
  };

  const handleDelete = async (id: string) => {
    try {
      await healthService.deleteSportSession(userId, id);
      setSessions(prev => prev.filter(s => s.id !== id));
    } catch {
      return;
    }
  };

  return (
    <section className="hero health-bg is-fullheight">
      <div className="hero-body" style={{ alignItems: 'flex-start', paddingTop: '2rem' }}>
        <div style={{ width: '100%' }}>
          <div className="page-header level is-mobile" style={{ marginBottom: '1rem' }}>
            <div className="level-left" style={{ gap: '.625rem' }}>
              <BackButton label="←" />
              <div>
                <p className="dash-sub is-size-7 has-text-weight-bold is-uppercase">Santé</p>
                <p className="dash-name">🏋️ Sport</p>
              </div>
            </div>
          </div>
          <hr />

          <div className="box health-box" style={{ marginTop: '1rem' }}>
            <div className="columns is-mobile" style={{ width: '100%', margin: 0 }}>
              <div className="column" style={{ padding: '.25rem' }}>
                <div className="pond-result" style={{ padding: '1rem' }}>
                  <div className="pond-result-val" style={{ color: 'var(--h-water)', fontSize: '1.9rem' }}>{todayBurned}</div>
                  <div className="pond-result-label">Calories brûlées aujourd'hui</div>
                </div>
              </div>
              <div className="column" style={{ padding: '.25rem' }}>
                <div className="pond-result" style={{ padding: '1rem' }}>
                  <div className="pond-result-val" style={{ color: 'var(--h-sober)', fontSize: '1.9rem' }}>{weekBurned}</div>
                  <div className="pond-result-label">Calories brûlées sur 7 jours</div>
                </div>
              </div>
            </div>

            <div className="field">
              <label className="label health-label">Activité</label>
              <div className="mood-picker" style={{ justifyContent: 'space-between', flexWrap: 'wrap' }}>
                {SPORT_TYPES.map(t => (
                  <button
                    key={t.value}
                    className={`button mood-btn${sportType === t.value ? ' is-active' : ''}`}
                    style={sportType === t.value ? { '--tc': 'var(--h-water)' } as React.CSSProperties : {}}
                    onClick={() => setSportType(t.value)}
                    type="button"
                  >
                    {t.emoji}
                  </button>
                ))}
              </div>
              <p className="health-subtitle" style={{ marginTop: '.5rem' }}>{selectedSport.label} • MET {selectedSport.met}</p>
            </div>

            <div className="columns is-mobile" style={{ width: '100%', margin: 0 }}>
              <div className="column" style={{ padding: '.25rem' }}>
                <div className="field">
                  <label className="label health-label">Durée (min)</label>
                  <input
                    className="input health-input"
                    type="number"
                    min={1}
                    step={1}
                    value={durationMin}
                    onChange={e => setDurationMin(Number(e.target.value))}
                  />
                </div>
              </div>
              <div className="column" style={{ padding: '.25rem' }}>
                <div className="field">
                  <label className="label health-label">Poids (kg)</label>
                  <input
                    className="input health-input"
                    type="number"
                    min={20}
                    step={1}
                    value={weightKg}
                    onChange={e => setWeightKg(Number(e.target.value))}
                  />
                </div>
              </div>
            </div>

            <div className="notification health-empty" style={{ background: 'rgba(255,255,255,.03)', border: '1px solid var(--border)', color: 'var(--txt)', padding: '1rem' }}>
              Estimation actuelle : <strong style={{ color: 'var(--h-water)' }}>{caloriesPreview} kcal</strong>
            </div>

            <div className="field">
              <label className="label health-label">Note (optionnel)</label>
              <textarea
                className="textarea health-input"
                rows={2}
                value={note}
                placeholder="Intensité, distance, sensations..."
                onChange={e => setNote(e.target.value)}
              />
            </div>

            <div style={{ display: 'flex', gap: '.75rem', flexWrap: 'wrap' }}>
              <button
                className="button health-btn-submit"
                onClick={handleSave}
                disabled={durationMin <= 0 || weightKg <= 0}
                style={{ '--tc': 'var(--h-water)', '--tg': 'var(--h-water-glow)' } as React.CSSProperties}
              >
                {editingSessionId ? 'Mettre à jour la séance' : 'Ajouter la séance'}
              </button>
              {editingSessionId && (
                <button
                  className="button"
                  style={{ background: 'transparent', border: '1px solid var(--border)', color: 'var(--txt-faint)' }}
                  type="button"
                  onClick={handleCancelEdit}
                >
                  Annuler
                </button>
              )}
            </div>

            <hr style={{ margin: 0 }} />

            <div>
              <p className="health-subtitle" style={{ marginBottom: '.75rem' }}>Séances enregistrées</p>
              {todaySessions.length === 0 ? (
                <div className="notification health-empty">Aucune séance enregistrée aujourd'hui.</div>
              ) : (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '.625rem' }}>
                  {todaySessions.map(session => {
                    const sport = SPORT_TYPES.find(t => t.value === session.sportType);
                    return (
                      <div key={session.id} className="health-item" style={{ justifyContent: 'space-between', alignItems: 'flex-start' }}>
                        <div className="health-item-info">
                          <div className="health-item-title">
                            {sport?.emoji} {sport?.label ?? session.sportType ?? 'Sport'}
                          </div>
                          <div className="health-item-meta">
                            {session.duration} min{session.intensity ? ` • ${session.intensity}` : ''}{session.note ? ` • ${session.note}` : ''}
                          </div>
                        </div>
                        <div style={{ display: 'flex', alignItems: 'center', gap: '.5rem' }}>
                          <span style={{ fontWeight: 700, color: 'var(--h-water)', whiteSpace: 'nowrap' }}>{session.calories} kcal</span>
                          <button className="button" style={{ background: 'transparent', border: '1px solid rgba(220,53,69,0.12)', color: 'var(--danger)' }} onClick={() => handleDelete(session.id)}>suppr</button>
                          <button className="button" style={{ background: 'transparent', border: '1px solid rgba(33,150,243,0.16)', color: 'var(--link)' }} onClick={() => handleEdit(session)}>modifier</button>
                        </div>
                      </div>
                    );
                  })}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default SportPage;
