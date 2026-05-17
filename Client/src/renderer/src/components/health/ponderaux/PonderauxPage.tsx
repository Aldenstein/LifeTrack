// ============================================================
// PonderauxPage.tsx  Calculateur pondéral en onglets
// IMC / Métabolisme Basal / TDEE-NEAT / Poids idéal
// Calculs uniquement, pas de persistance  outil de référence
// ============================================================

import React, { useState } from 'react';
import BackButton from '../../BackButton';
import { Gender, ActivityLevel } from '../../../types/health';
import '../../../styles/health.css';

type Tab = 'imc' | 'mb' | 'tdee' | 'ideal';

const ACTIVITY: { key: ActivityLevel; label: string; factor: number }[] = [
  { key: 'sedentary',   label: 'Sédentaire (aucun sport)',          factor: 1.2 },
  { key: 'light',       label: 'Léger (1–3j/sem)',                  factor: 1.375 },
  { key: 'moderate',    label: 'Modéré (3–5j/sem)',                 factor: 1.55 },
  { key: 'active',      label: 'Actif (6–7j/sem)',                  factor: 1.725 },
  { key: 'very_active', label: 'Très actif (sport intense quotidien)', factor: 1.9 },
];

function calcIMC(weight: number, height: number): number {
  return weight / Math.pow(height / 100, 2);
}
function imcCategory(imc: number): { label: string; color: string } {
  if (imc < 18.5) return { label: 'Sous-poids', color: 'var(--h-sleep)' };
  if (imc < 25)   return { label: 'Normal', color: 'var(--h-sober)' };
  if (imc < 30)   return { label: 'Surpoids', color: 'var(--h-mood)' };
  return           { label: 'Obésité', color: 'var(--h-alcohol)' };
}

// Formule Mifflin-St Jeor
function calcMB(gender: Gender, weight: number, height: number, age: number): number {
  if (gender === 'male') return 10 * weight + 6.25 * height - 5 * age + 5;
  return 10 * weight + 6.25 * height - 5 * age - 161;
}

// Lorentz et Devine
function calcIdeal(gender: Gender, height: number): { lorentz: number; devine: number } {
  const h = height - 150;
  const lorentz = gender === 'male'
    ? height - 100 - h / 4
    : height - 100 - h / 2.5;
  const devine = gender === 'male'
    ? 50 + 2.3 * ((height - 152.4) / 2.54)
    : 45.5 + 2.3 * ((height - 152.4) / 2.54);
  return { lorentz: Math.round(lorentz * 10) / 10, devine: Math.round(devine * 10) / 10 };
}

const PonderauxPage: React.FC = () => {
  const [tab, setTab] = useState<Tab>('imc');

  // Champs partagés
  const [gender, setGender]   = useState<Gender>('male');
  const [weight, setWeight]   = useState(70);
  const [height, setHeight]   = useState(175);
  const [age,    setAge]      = useState(30);
  const [activity, setActivity] = useState<ActivityLevel>('moderate');

  const imc    = calcIMC(weight, height);
  const imcCat = imcCategory(imc);
  const mb     = calcMB(gender, weight, height, age);
  const factor = ACTIVITY.find(a => a.key === activity)!.factor;
  const tdee   = mb * factor;
  const { lorentz, devine } = calcIdeal(gender, height);

  const NumInput = ({ label, value, onChange, min, max, unit }: {
    label: string; value: number; onChange: (v: number) => void;
    min: number; max: number; unit?: string;
  }) => (
    <div className="field">
      <label className="label health-label">{label}{unit && ` (${unit})`}</label>
      <div className="control">
        <input className="input health-input" type="number"
          min={min} max={max} value={value}
          onChange={e => onChange(Number(e.target.value))} />
      </div>
    </div>
  );

  return (
    <section className="hero health-bg is-fullheight">
      <div className="hero-body" style={{ alignItems: 'flex-start', paddingTop: '2rem' }}>
        <div style={{ width: '100%' }}>
          <div className="page-header level is-mobile" style={{ marginBottom: '1rem' }}>
            <div className="level-left" style={{ gap: '.625rem' }}>
              <BackButton label="←" />
              <div>
                <p className="dash-sub is-size-7 has-text-weight-bold is-uppercase">Santé</p>
                <p className="dash-name">⚖️ Calculs pondéraux</p>
              </div>
            </div>
          </div>
          <hr />

          <div className="box health-box health-box--wide" style={{ marginTop: '1rem' }}>

            {/* Onglets */}
            <div className="tabs health-tabs" style={{ '--tc': 'var(--h-weight)' } as React.CSSProperties}>
              <ul>
                {([['imc','IMC'], ['mb','Métabolisme'], ['tdee','NEAT/TDEE'], ['ideal','Poids idéal']] as [Tab,string][]).map(([k, l]) => (
                  <li key={k} className={tab === k ? 'is-active' : ''}>
                    <a onClick={() => setTab(k)}>{l}</a>
                  </li>
                ))}
              </ul>
            </div>

            {/* Champs communs */}
            <div className="columns is-multiline" style={{ margin: 0 }}>
              <div className="column is-half" style={{ padding: '0 .25rem' }}>
                <div className="field">
                  <label className="label health-label">Genre</label>
                  <div style={{ display: 'flex', gap: '.5rem' }}>
                    {(['male', 'female'] as Gender[]).map(g => (
                      <button key={g}
                        className={`button is-small${gender === g ? ' is-active' : ''}`}
                        style={gender === g
                          ? { background: 'var(--h-weight)', color: '#0f0e17', border: 'none', borderRadius: '8px', flex: 1, fontWeight: 700 }
                          : { background: 'var(--surface-up)', border: '1px solid var(--border)', color: 'var(--txt-muted)', borderRadius: '8px', flex: 1 }
                        }
                        onClick={() => setGender(g)}
                      >
                        {g === 'male' ? '♂ Homme' : '♀ Femme'}
                      </button>
                    ))}
                  </div>
                </div>
              </div>
              <div className="column is-half" style={{ padding: '0 .25rem' }}>
                <NumInput label="Âge" value={age} onChange={setAge} min={10} max={120} unit="ans" />
              </div>
              <div className="column is-half" style={{ padding: '0 .25rem' }}>
                <NumInput label="Taille" value={height} onChange={setHeight} min={100} max={250} unit="cm" />
              </div>
              <div className="column is-half" style={{ padding: '0 .25rem' }}>
                <NumInput label="Poids" value={weight} onChange={setWeight} min={30} max={300} unit="kg" />
              </div>
            </div>

            <hr style={{ margin: 0 }} />

            {/* Résultats par onglet */}
            {tab === 'imc' && (
              <div>
                <div className="pond-result">
                  <div className="pond-result-val" style={{ color: imcCat.color }}>
                    {imc.toFixed(1)}
                  </div>
                  <div className="pond-result-label">Indice de Masse Corporelle</div>
                  <span className="pond-result-badge" style={{ background: `${imcCat.color}20`, color: imcCat.color }}>
                    {imcCat.label}
                  </span>
                </div>
                <div style={{ marginTop: '.75rem', fontSize: '.75rem', color: 'var(--txt-muted)', lineHeight: 1.6 }}>
                  <p>{'< 18.5'} : Sous-poids · 18.5–24.9 : Normal · 25–29.9 : Surpoids · {'≥ 30'} : Obésité</p>
                  <p style={{ marginTop: '.5rem', color: 'var(--txt-faint)', fontSize: '.7rem' }}>
                    L'IMC est un indicateur statistique  il ne remplace pas l'avis d'un professionnel de santé.
                  </p>
                </div>
              </div>
            )}

            {tab === 'mb' && (
              <div className="pond-result">
                <div className="pond-result-val">{Math.round(mb)}</div>
                <div className="pond-result-label">kcal / jour (Métabolisme Basal)</div>
                <p style={{ fontSize: '.75rem', color: 'var(--txt-muted)', marginTop: '.5rem' }}>
                  Énergie minimale au repos  formule Mifflin-St Jeor
                </p>
              </div>
            )}

            {tab === 'tdee' && (
              <div>
                <div className="field" style={{ marginBottom: '.75rem' }}>
                  <label className="label health-label">Niveau d'activité</label>
                  <div className="health-select">
                    <div className="select is-fullwidth">
                      <select className="health-input"
                        value={activity}
                        onChange={e => setActivity(e.target.value as ActivityLevel)}>
                        {ACTIVITY.map(a => (
                          <option key={a.key} value={a.key}>{a.label} (×{a.factor})</option>
                        ))}
                      </select>
                    </div>
                  </div>
                </div>
                <div className="pond-result">
                  <div className="pond-result-val">{Math.round(tdee)}</div>
                  <div className="pond-result-label">kcal / jour (TDEE)</div>
                </div>
                <div style={{ marginTop: '.625rem', display: 'flex', gap: '.5rem' }}>
                  {[['Perte', -500], ['Maintien', 0], ['Prise', +500]].map(([lbl, delta]) => (
                    <div key={String(lbl)} style={{ flex: 1, background: 'var(--surface-up)', border: '1px solid var(--border)', borderRadius: '10px', padding: '.5rem', textAlign: 'center' }}>
                      <div style={{ fontSize: '.875rem', fontWeight: 700, color: 'var(--h-weight)' }}>
                        {Math.round(tdee + Number(delta))}
                      </div>
                      <div style={{ fontSize: '.65rem', color: 'var(--txt-muted)', textTransform: 'uppercase', letterSpacing: '.06em' }}>{lbl}</div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {tab === 'ideal' && (
              <div style={{ display: 'flex', gap: '.75rem' }}>
                {[['Lorentz', lorentz], ['Devine', devine]].map(([name, val]) => (
                  <div key={String(name)} className="pond-result" style={{ flex: 1 }}>
                    <div className="pond-result-val">{val} kg</div>
                    <div className="pond-result-label">Formule {name}</div>
                  </div>
                ))}
              </div>
            )}

          </div>
        </div>
      </div>
    </section>
  );
};

export default PonderauxPage;