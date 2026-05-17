// ============================================================
// HydratationPage.tsx  Suivi de l'hydratation quotidienne
// Boutons +ml, jauge de progression, historique 7 jours
// Données stockées en localStorage sous "health_hydra"
// ============================================================

import React, { useState, useEffect } from 'react';
import BackButton from '../../BackButton';
import { HydratationEntry } from '../../../types/health';
import { healthService } from '@/services/healthService';
import { useUserStore } from '@/store/userStore';
import '../../../styles/health.css';

const TODAY = new Date().toISOString().slice(0, 10);

// Récupère ou initialise l'entrée du jour
function getTodayEntry(entries: HydratationEntry[]): HydratationEntry {
  return entries.find(e => e.date === TODAY) ?? { date: TODAY, totalMl: 0, goal: 2000 };
}

function last7Days(): string[] {
  return Array.from({ length: 7 }, (_, i) => {
    const d = new Date();
    d.setDate(d.getDate() - (6 - i));
    return d.toISOString().slice(0, 10);
  });
}

const QUICK_ADD = [150, 250, 330, 500];

const HydratationPage: React.FC = () => {
  const userId = useUserStore(s => s.profile?.id) ?? 0;
  const [entries, setEntries] = useState<HydratationEntry[]>([]);

  const [goal, setGoal] = useState(2000);
  const todayEntry = getTodayEntry(entries);
  const pct = Math.min((todayEntry.totalMl / goal) * 100, 100);

  useEffect(() => {
    void (async () => {
      try {
        const data = await healthService.getHydratationEntries(userId);
        setEntries(data);
        setGoal(getTodayEntry(data).goal);
      } catch {
        setEntries([]);
      }
    })();
  }, [userId]);

  useEffect(() => {
    setGoal(getTodayEntry(entries).goal);
  }, [entries]);

  // Ajoute des ml à l'entrée du jour
  const addMl = (ml: number) => {
    setEntries(prev => {
      const exists = prev.find(e => e.date === TODAY);
      if (exists) {
        const next = prev.map(e =>
          e.date === TODAY ? { ...e, totalMl: e.totalMl + ml } : e
        );
        const updated = next.find(e => e.date === TODAY);
        if (updated) void healthService.upsertHydratationEntry(userId, updated);
        return next;
      }
      const next = [...prev, { date: TODAY, totalMl: ml, goal }];
      const updated = next.find(e => e.date === TODAY);
      if (updated) void healthService.upsertHydratationEntry(userId, updated);
      return next;
    });
  };

  // Réinitialise le jour courant
  const resetToday = () => {
    setEntries(prev => {
      const next = prev.map(e => e.date === TODAY ? { ...e, totalMl: 0 } : e);
      const current = next.find(e => e.date === TODAY);
      if (current) void healthService.upsertHydratationEntry(userId, current);
      return next;
    });
  };

  const days = last7Days();

  return (
    <section className="hero health-bg is-fullheight">
      <div className="hero-body" style={{ alignItems: 'flex-start', paddingTop: '2rem' }}>
        <div style={{ width: '100%' }}>
          <div className="page-header level is-mobile" style={{ marginBottom: '1rem' }}>
            <div className="level-left" style={{ gap: '.625rem' }}>
              <BackButton label="←" />
              <div>
                <p className="dash-sub is-size-7 has-text-weight-bold is-uppercase">Santé</p>
                <p className="dash-name">💧 Hydratation</p>
              </div>
            </div>
          </div>
          <hr />

          <div className="box health-box" style={{ marginTop: '1rem' }}>

            {/* Jauge principale */}
            <div className="hydra-gauge-wrap">
              <span className="hydra-gauge-val">
                {todayEntry.totalMl >= 1000
                  ? `${(todayEntry.totalMl / 1000).toFixed(1)}L`
                  : `${todayEntry.totalMl}ml`}
              </span>
              <div className="hydra-track" style={{ width: '100%' }}>
                <div className="hydra-fill" style={{ width: `${pct}%` }} />
              </div>
              <span className="hydra-gauge-goal">
                Objectif : {goal >= 1000 ? `${goal / 1000}L` : `${goal}ml`}
                &nbsp; <strong style={{ color: 'var(--h-water)' }}>{Math.round(pct)}%</strong>
              </span>
            </div>

            {/* Boutons d'ajout rapide */}
            <div>
              <p className="health-subtitle" style={{ marginBottom: '.625rem' }}>Ajouter</p>
              <div style={{ display: 'flex', gap: '.5rem', flexWrap: 'wrap' }}>
                {QUICK_ADD.map(ml => (
                  <button key={ml} className="button hydra-add-btn" onClick={() => addMl(ml)}>
                    +{ml}ml
                  </button>
                ))}
              </div>
            </div>

            {/* Objectif configurable */}
            <div className="field">
              <label className="label health-label">Objectif journalier (ml)</label>
              <div className="control">
                <input
                  className="input health-input"
                  type="number"
                  min={500} max={5000} step={250}
                  value={goal}
                  onChange={e => setGoal(Number(e.target.value))}
                />
              </div>
            </div>

            {/* Historique 7 jours */}
            <div>
              <p className="health-subtitle" style={{ marginBottom: '.625rem' }}>7 derniers jours</p>
              <div style={{ display: 'flex', gap: '4px', alignItems: 'flex-end', height: '48px' }}>
                {days.map(d => {
                  const e = entries.find(x => x.date === d);
                  const g = e?.goal ?? goal;
                  const pctD = e ? Math.min((e.totalMl / g) * 100, 100) : 0;
                  return (
                    <div key={d} style={{ flex: 1, display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 2, height: '100%', justifyContent: 'flex-end' }}>
                      <div style={{
                        width: '100%', height: `${Math.max(pctD, 5)}%`,
                        borderRadius: '4px 4px 0 0',
                        background: pctD >= 100 ? 'var(--h-sober)' : pctD >= 60 ? 'var(--h-water)' : 'var(--surface-top)',
                        transition: 'height .6s cubic-bezier(.16,1,.3,1)'
                      }} />
                      <span style={{ fontSize: '.6rem', color: 'var(--txt-faint)' }}>{d.slice(8)}</span>
                    </div>
                  );
                })}
              </div>
            </div>

            {/* Reset du jour */}
            <button
              className="button is-small"
              style={{ background: 'transparent', border: '1px solid var(--border)', color: 'var(--txt-faint)', borderRadius: '8px', fontSize: '.75rem' }}
              onClick={resetToday}
            >
              Réinitialiser aujourd'hui
            </button>

          </div>
        </div>
      </div>
    </section>
  );
};

export default HydratationPage;