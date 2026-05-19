// ============================================================
// SommeilPage.tsx  Sous-module Sommeil
// Log d'une nuit : heure coucher/lever + qualité 1→5
// Historique 7 jours en mini-barres colorées + moyenne
// Données stockées en localStorage sous "health_sleep"
// ============================================================

import React, { useState, useEffect } from 'react';

import BackButton from '../../BackButton';
import { SleepEntry, SleepQuality } from '../../../types/health';
import { healthService } from '@/services/healthService';
import { useUserStore } from '@/store/userStore';
import '../../../styles/health.css';

// Emojis et libellés de qualité
const QUALITY_LABELS: Record<SleepQuality, string> = {
  1: '😩', 2: '😕', 3: '😐', 4: '🙂', 5: '😴'
};

// Calcule la durée en heures depuis deux string "HH:MM"
function calcDuration(bed: string, wake: string): number {
  const [bh, bm] = bed.split(':').map(Number);
  const [wh, wm] = wake.split(':').map(Number);
  let mins = (wh * 60 + wm) - (bh * 60 + bm);
  if (mins < 0) mins += 24 * 60; // passage minuit
  return mins / 60;
}

// Formate une durée en "7h30"
function fmtDuration(h: number): string {
  const hh = Math.floor(h);
  const mm = Math.round((h - hh) * 60);
  return `${hh}h${mm.toString().padStart(2, '0')}`;
}

// Génère les 7 derniers jours ISO
function last7Days(): string[] {
  return Array.from({ length: 7 }, (_, i) => {
    const d = new Date();
    d.setDate(d.getDate() - (6 - i));
    return d.toISOString().slice(0, 10);
  });
}

const SommeilPage: React.FC = () => {
  const userId = useUserStore(s => s.profile?.id) ?? 0;

  const today = new Date().toISOString().slice(0, 10);

  // Charge les entrées depuis localStorage
  const [entries, setEntries] = useState<SleepEntry[]>([]);

  // Formulaire de saisie
  const [bedTime,  setBedTime]  = useState('23:00');
  const [wakeTime, setWakeTime] = useState('07:00');
  const [quality,  setQuality]  = useState<SleepQuality>(3);
  const [note,     setNote]     = useState('');

  // Persiste à chaque modification
  useEffect(() => {
    void (async () => {
      try {
        setEntries(await healthService.getSleepEntries(userId));
      } catch {
        setEntries([]);
      }
    })();
  }, [userId]);

  // Entrée du jour déjà présente ?
  const todayEntry = entries.find(e => e.date === today);

  // Sauvegarde ou mise à jour de l'entrée du jour
  const handleSave = () => {
    const durationHours = calcDuration(bedTime, wakeTime);
    const newEntry: SleepEntry = {
      id: todayEntry?.id ?? Date.now().toString(),
      date: today,
      time: bedTime.length === 5 ? bedTime + ':00' : bedTime,
      duration: Math.round(durationHours * 60),
      quality,
      is_restful: quality >= 4,
      note,
    };
    setEntries(prev =>
      todayEntry
        ? prev.map(e => e.id === todayEntry.id ? newEntry : e)
        : [...prev, newEntry]
    );
    void healthService.upsertSleepEntry(userId, newEntry);
  };

  // Données pour l'affichage historique
  const days = last7Days();
  const maxDur = Math.max(...days.map(d => {
    const e = entries.find(x => x.date === d);
    return e ? e.duration / 60 : 0;
  }), 8);

  const filled = days.filter(d => entries.find(x => x.date === d));
  const avgDur = filled.length
    ? filled.reduce((s, d) => {
        const e = entries.find(x => x.date === d)!;
        return s + e.duration / 60;
      }, 0) / filled.length
    : 0;

  return (
    <section className="hero health-bg is-fullheight">
      <div className="hero-body" style={{ alignItems: 'flex-start', paddingTop: '2rem' }}>
        <div style={{ width: '100%' }}>

          {/* En-tête */}
          <div className="page-header level is-mobile" style={{ marginBottom: '1rem' }}>
            <div className="level-left" style={{ gap: '.625rem' }}>
              <BackButton label="←" />
              <div>
                <p className="dash-sub is-size-7 has-text-weight-bold is-uppercase">Santé</p>
                <p className="dash-name">😴 Sommeil</p>
              </div>
            </div>
          </div>
          <hr />

          <div className="box health-box" style={{ marginTop: '1rem' }}>

            {/* Historique 7 jours */}
            <div>
              <p className="health-subtitle" style={{ marginBottom: '.75rem' }}>
                7 derniers jours
              </p>
              <div style={{ display: 'flex', gap: '4px', alignItems: 'flex-end', height: '52px' }}>
                {days.map(d => {
                  const e = entries.find(x => x.date === d);
                  const dur = e ? e.duration / 60 : 0;
                  const pct = maxDur > 0 ? (dur / maxDur) * 100 : 0;
                  const qClass = e ? `q${e.quality}` : '';
                  return (
                    <div
                      key={d}
                      title={e ? `${fmtDuration(dur)}  ${QUALITY_LABELS[e.quality]}` : 'Aucune donnée'}
                      style={{ flex: 1, display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 2, height: '100%', justifyContent: 'flex-end' }}
                    >
                      <div
                        className={`sleep-bar ${qClass}`}
                        style={{ width: '100%', height: `${Math.max(pct, 5)}%`, borderRadius: '4px 4px 0 0' }}
                      />
                      <span style={{ fontSize: '.6rem', color: 'var(--txt-faint)' }}>
                        {d.slice(8)}
                      </span>
                    </div>
                  );
                })}
              </div>

              {/* Moyenne */}
              {avgDur > 0 && (
                <p className="sleep-avg" style={{ marginTop: '.5rem' }}>
                  Moyenne : <strong style={{ color: 'var(--h-sleep)' }}>{fmtDuration(avgDur)}</strong> par nuit
                </p>
              )}
            </div>

            <hr style={{ margin: '0' }} />

            {/* Formulaire de saisie */}
            <div>
              <p className="health-subtitle" style={{ marginBottom: '.75rem' }}>
                {todayEntry ? 'Modifier ce soir' : 'Enregistrer cette nuit'}
              </p>

              <div className="field">
                <label className="label health-label">Heure de coucher</label>
                <div className="control">
                  <input
                    className="input health-input"
                    type="time"
                    value={bedTime}
                    onChange={e => setBedTime(e.target.value)}
                  />
                </div>
              </div>

              <div className="field">
                <label className="label health-label">Heure de réveil</label>
                <div className="control">
                  <input
                    className="input health-input"
                    type="time"
                    value={wakeTime}
                    onChange={e => setWakeTime(e.target.value)}
                  />
                </div>
              </div>

              {/* Durée calculée en live */}
              <div style={{ background: 'var(--surface-up)', borderRadius: '10px', padding: '.625rem 1rem', textAlign: 'center', border: '1px solid var(--border)' }}>
                <span className="sleep-duration">{fmtDuration(calcDuration(bedTime, wakeTime))}</span>
                <span className="sleep-avg"> de sommeil</span>
              </div>

              {/* Qualité ressentie */}
              <div className="field" style={{ marginTop: '.75rem' }}>
                <label className="label health-label">Qualité ressentie</label>
                <div style={{ display: 'flex', gap: '.5rem', justifyContent: 'space-around' }}>
                  {([1, 2, 3, 4, 5] as SleepQuality[]).map(q => (
                    <button
                      key={q}
                      className={`button mood-btn${quality === q ? ' is-active' : ''}`}
                      style={quality === q ? { '--tc': 'var(--h-sleep)' } as React.CSSProperties : {}}
                      onClick={() => setQuality(q)}
                    >
                      {QUALITY_LABELS[q]}
                    </button>
                  ))}
                </div>
              </div>

              {/* Note optionnelle */}
              <div className="field">
                <label className="label health-label">Note (optionnel)</label>
                <div className="control">
                  <textarea
                    className="textarea health-input"
                    rows={2}
                    placeholder="Cauchemar, réveil nocturne..."
                    value={note}
                    onChange={e => setNote(e.target.value)}
                  />
                </div>
              </div>

              <div style={{ display: 'flex', gap: '.5rem' }}>
                <button className="button health-btn-submit is-fullwidth" onClick={handleSave}
                  style={{ '--tc': 'var(--h-sleep)', '--tg': 'var(--h-sleep-glow)' } as React.CSSProperties}>
                  {todayEntry ? 'Mettre à jour' : 'Enregistrer'}
                </button>

                {todayEntry && (
                  <button className="button" style={{ background: 'transparent', border: '1px solid rgba(220,53,69,0.12)', color: 'var(--danger)', borderRadius: '8px', fontSize: '.9rem' }}
                    onClick={async () => {
                      if (!confirm("Supprimer l'entrée de sommeil d'aujourd'hui ?")) return;
                      setEntries(prev => prev.filter(e => e.date !== today));
                      try { await healthService.deleteSleepEntry(userId, today); } catch {}
                    }}
                  >Supprimer</button>
                )}
              </div>
            </div>

          </div>
        </div>
      </div>
    </section>
  );
};

export default SommeilPage;
