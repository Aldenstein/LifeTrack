// ============================================================
// CoherencePage.tsx  Cohérence cardiaque & respiration guidée
// Bulle animée CSS, 4 programmes, chronomètre, guidage textuel
// Animation pilotée par JS via keyframes injectées dynamiquement
// ============================================================

import React, { useState, useEffect, useRef, useCallback } from 'react';
import BackButton from '../../BackButton';
import { BreathingProgram, BreathingProgramId } from '../../../types/health';
import '../../../styles/health.css';

// Définition des 4 programmes de respiration
const PROGRAMS: BreathingProgram[] = [
  {
    id: 'coherence', label: 'Cohérence 5/5',
    description: '5s inspire · 5s expire',
    inhale: 5, exhale: 5,
    color: '#9d93e0',
  },
  {
    id: 'energizing', label: 'Énergisant 4/2/4',
    description: '4s inspire · 2s rétention · 4s expire',
    inhale: 4, holdIn: 2, exhale: 4,
    color: '#d4a853',
  },
  {
    id: 'calming', label: 'Calmant 4/7/8',
    description: '4s inspire · 7s rétention · 8s expire',
    inhale: 4, holdIn: 7, exhale: 8,
    color: '#4fc3d4',
  },
  {
    id: 'box', label: 'Box Breathing',
    description: '4s inspire · 4s · 4s expire · 4s',
    inhale: 4, holdIn: 4, exhale: 4, holdOut: 4,
    color: '#6aab7a',
  },
];

type Phase = 'idle' | 'inhale' | 'holdIn' | 'exhale' | 'holdOut';
const PHASE_LABELS: Record<Phase, string> = {
  idle: 'Appuyez sur ▶ pour commencer',
  inhale:  'Inspirez…',
  holdIn:  'Retenez…',
  exhale:  'Expirez…',
  holdOut: 'Pause…',
};

const SESSION_OPTS = [5, 10, 15, 20];

const CoherencePage: React.FC = () => {
  const [progId,    setProgId]   = useState<BreathingProgramId>('coherence');
  const [running,   setRunning]  = useState(false);
  const [phase,     setPhase]    = useState<Phase>('idle');
  const [cycles,    setCycles]   = useState(0);
  const [sessionMin, setSessionMin] = useState(5);
  const [elapsed,   setElapsed]  = useState(0);  // secondes écoulées
  const [bubbleScale, setBubbleScale] = useState(1);

  const prog = PROGRAMS.find(p => p.id === progId)!;
  const sessionSecs = sessionMin * 60;
  const remaining = Math.max(sessionSecs - elapsed, 0);

  // Refs pour éviter les stale closures dans les intervalles
  const phaseRef     = useRef<Phase>('idle');
  const phaseSecs    = useRef(0);
  const cycleRef     = useRef(0);
  const timerRef     = useRef<ReturnType<typeof setInterval> | null>(null);

  const fmtTime = (s: number) => `${Math.floor(s / 60).toString().padStart(2, '0')}:${(s % 60).toString().padStart(2, '0')}`;

  // Séquence des phases selon le programme
  const getNextPhase = useCallback((current: Phase): Phase => {
    if (current === 'idle' || current === 'exhale' || current === 'holdOut') return 'inhale';
    if (current === 'inhale') return prog.holdIn ? 'holdIn' : 'exhale';
    if (current === 'holdIn') return 'exhale';
    if (current === 'exhale') return prog.holdOut ? 'holdOut' : 'inhale';
    return 'inhale';
  }, [prog]);

  // Durée d'une phase
  const phaseDuration = useCallback((p: Phase): number => {
    if (p === 'inhale')  return prog.inhale;
    if (p === 'holdIn')  return prog.holdIn ?? 0;
    if (p === 'exhale')  return prog.exhale;
    if (p === 'holdOut') return prog.holdOut ?? 0;
    return 0;
  }, [prog]);

  // Démarre / pause
  const toggleRun = () => {
    if (running) {
      setRunning(false);
      setPhase('idle');
      phaseRef.current = 'idle';
    } else {
      setRunning(true);
      phaseRef.current = 'inhale';
      setPhase('inhale');
      phaseSecs.current = 0;
    }
  };

  // Reset complet
  const handleReset = () => {
    setRunning(false);
    setPhase('idle');
    setCycles(0);
    setElapsed(0);
    setBubbleScale(1);
    phaseRef.current = 'idle';
    phaseSecs.current = 0;
    cycleRef.current = 0;
  };

  // Boucle principale  1 tick/seconde
  useEffect(() => {
    if (!running) {
      if (timerRef.current) clearInterval(timerRef.current);
      return;
    }

    timerRef.current = setInterval(() => {
      // Temps de séance
      setElapsed(prev => {
        const next = prev + 1;
        if (next >= sessionSecs) {
          setRunning(false);
          setPhase('idle');
          return prev;
        }
        return next;
      });

      phaseSecs.current += 1;
      const dur = phaseDuration(phaseRef.current as Phase);

      if (phaseSecs.current >= dur) {
        // Passage à la phase suivante
        const next = getNextPhase(phaseRef.current as Phase) as Phase;
        phaseRef.current = next;
        setPhase(next);
        phaseSecs.current = 0;

        // Compte un cycle complet à chaque début d'inspiration
        if (next === 'inhale') {
          cycleRef.current += 1;
          setCycles(cycleRef.current);
        }
      }

      // Taille de la bulle : grande à l'inspire, petite à l'expire
      setPhase(p => {
        const sc = (p === 'inhale') ? 1 + (phaseSecs.current / dur) * 0.6
          : (p === 'exhale') ? 1.6 - (phaseSecs.current / dur) * 0.6
          : p === 'holdIn' ? 1.6 : 1.0;
        setBubbleScale(sc);
        return p;
      });

    }, 1000);

    return () => { if (timerRef.current) clearInterval(timerRef.current); };
  }, [running, prog, sessionSecs, getNextPhase, phaseDuration]);

  return (
    <section className="hero health-bg is-fullheight">
      <div className="hero-body" style={{ alignItems: 'flex-start', paddingTop: '2rem' }}>
        <div style={{ width: '100%' }}>
          <div className="page-header level is-mobile" style={{ marginBottom: '1rem' }}>
            <div className="level-left" style={{ gap: '.625rem' }}>
              <BackButton label="←" />
              <div>
                <p className="dash-sub is-size-7 has-text-weight-bold is-uppercase">Santé</p>
                <p className="dash-name">🫁 Cohérence cardiaque</p>
              </div>
            </div>
          </div>
          <hr />

          <div className="box health-box" style={{ marginTop: '1rem' }}>

            {/* Sélecteur de programme */}
            <div>
              <p className="health-subtitle" style={{ marginBottom: '.5rem' }}>Programme</p>
              <div className="breath-program-grid">
                {PROGRAMS.map(p => (
                  <button
                    key={p.id}
                    className={`button breath-prog-btn${progId === p.id ? ' is-active' : ''}`}
                    style={{ '--bc': p.color } as React.CSSProperties}
                    onClick={() => { if (!running) setProgId(p.id as BreathingProgramId); }}
                    disabled={running}
                  >
                    <span className="breath-prog-name">{p.label}</span>
                    <span className="breath-prog-desc">{p.description}</span>
                  </button>
                ))}
              </div>
            </div>

            {/* Durée de séance */}
            <div>
              <p className="health-subtitle" style={{ marginBottom: '.5rem' }}>Durée de séance</p>
              <div style={{ display: 'flex', gap: '.375rem' }}>
                {SESSION_OPTS.map(m => (
                  <button key={m}
                    style={sessionMin === m && !running
                      ? { flex: 1, background: prog.color, color: '#0f0e17', border: 'none', borderRadius: '8px', fontWeight: 700, padding: '.4rem .5rem', fontSize: '.8rem' }
                      : { flex: 1, background: 'var(--surface-up)', border: '1px solid var(--border)', color: 'var(--txt-muted)', borderRadius: '8px', padding: '.4rem .5rem', fontSize: '.8rem' }
                    }
                    onClick={() => { if (!running) setSessionMin(m); }}
                    disabled={running}
                  >{m} min</button>
                ))}
              </div>
            </div>

            <hr style={{ margin: 0 }} />

            {/* Bulle animée */}
            <div className="breath-wrap">
              <div className="breath-bubble-outer">
                <div className="breath-bubble-bg" />
                <div
                  className="breath-bubble"
                  style={{
                    '--bc': prog.color,
                    '--bc-light': `${prog.color}cc`,
                    '--bc-glow': `${prog.color}50`,
                    transform: `scale(${bubbleScale})`,
                    transition: `transform ${running ? '1s' : '.4s'} ease-in-out`,
                  } as React.CSSProperties}
                />
              </div>

              {/* Texte guidage */}
              <p className={`breath-guide-text${running ? ' active' : ''}`}
                style={{ '--bc': prog.color } as React.CSSProperties}>
                {PHASE_LABELS[phase]}
              </p>

              {/* Chronomètre */}
              <div className="breath-timer">{fmtTime(remaining)}</div>
              <p className="breath-cycles">
                {cycles > 0 ? `${cycles} cycle${cycles > 1 ? 's' : ''} complété${cycles > 1 ? 's' : ''}` : 'Prêt'}
              </p>
            </div>

            {/* Contrôles */}
            <div style={{ display: 'flex', justifyContent: 'center', gap: '1rem' }}>
              <button
                className="button breath-ctrl-btn"
                style={{ '--bc': prog.color, '--bc-glow': `${prog.color}50` } as React.CSSProperties}
                onClick={handleReset}
                title="Réinitialiser"
              >↺</button>
              <button
                className="button breath-ctrl-btn is-primary"
                style={{ '--bc': prog.color, '--bc-glow': `${prog.color}50` } as React.CSSProperties}
                onClick={toggleRun}
                title={running ? 'Pause' : 'Démarrer'}
              >
                {running ? '⏸' : '▶'}
              </button>
            </div>

          </div>
        </div>
      </div>
    </section>
  );
};

export default CoherencePage;