// ============================================================
// AlcoolemiePage.tsx  Calculateur d'alcoolémie (Widmark)
// Formulaire sexe/poids/boissons/durée → résultat en g/L
// Pas de persistance  outil ponctuel avec disclaimer
// ============================================================

import React, { useState } from 'react';
import BackButton from '../../BackButton';
import { Gender, DrinkType, DrinkItem } from '../../../types/health';
import '../../../styles/health.css';

// Teneur en alcool par verre standard (en grammes d'alcool pur)
const DRINK_DEF: Record<DrinkType, { label: string; emoji: string; grams: number }> = {
  beer:     { label: 'Bière 25cl',   emoji: '🍺', grams: 10 },
  wine:     { label: 'Vin 12cl',     emoji: '🍷', grams: 12 },
  shot:     { label: 'Shot 4cl',     emoji: '🥃', grams: 12 },
  cocktail: { label: 'Cocktail',     emoji: '🍹', grams: 14 },
  cider:    { label: 'Cidre 25cl',   emoji: '🫧', grams: 8  },
};

// Coefficient de distribution Widmark
const r = { male: 0.68, female: 0.55 };
// Élimination horaire moyenne
const ELIMINATION = 0.15;

function calcAlco(gender: Gender, weight: number, drinks: DrinkItem[], hours: number): number {
  const totalG = drinks.reduce((s, d) => s + DRINK_DEF[d.type].grams * d.quantity, 0);
  const taux = totalG / (weight * r[gender]) - ELIMINATION * hours;
  return Math.max(taux, 0);
}

function getLevel(taux: number): { cls: string; label: string; color: string } {
  if (taux === 0) return { cls: '', label: '', color: 'var(--txt-muted)' };
  if (taux < 0.5) return { cls: 'green',  label: 'Légal (< 0.5 g/L)', color: 'var(--h-sober)' };
  if (taux < 0.8) return { cls: 'orange', label: 'Illégal ≥ 0.5 g/L', color: 'var(--h-mood)' };
  return              { cls: 'red',    label: 'Dangereux ≥ 0.8 g/L', color: 'var(--h-alcohol)' };
}

const AlcoolemiePage: React.FC = () => {
  const [gender, setGender] = useState<Gender>('male');
  const [weight, setWeight] = useState(70);
  const [hours,  setHours]  = useState(0);
  const [drinks, setDrinks] = useState<DrinkItem[]>([]);

  const addDrink = (type: DrinkType) => {
    const existing = drinks.find(d => d.type === type);
    if (existing) {
      setDrinks(drinks.map(d => d.type === type ? { ...d, quantity: d.quantity + 1 } : d));
    } else {
      setDrinks([...drinks, { type, quantity: 1 }]);
    }
  };

  const removeDrink = (type: DrinkType) => {
    setDrinks(drinks
      .map(d => d.type === type ? { ...d, quantity: d.quantity - 1 } : d)
      .filter(d => d.quantity > 0)
    );
  };

  const taux = calcAlco(gender, weight, drinks, hours);
  const level = getLevel(taux);
  const timeToZero = taux > 0 ? Math.ceil(taux / ELIMINATION * 10) / 10 : 0;

  return (
    <section className="hero health-bg is-fullheight">
      <div className="hero-body" style={{ alignItems: 'flex-start', paddingTop: '2rem' }}>
        <div style={{ width: '100%' }}>
          <div className="page-header level is-mobile" style={{ marginBottom: '1rem' }}>
            <div className="level-left" style={{ gap: '.625rem' }}>
              <BackButton label="←" />
              <div>
                <p className="dash-sub is-size-7 has-text-weight-bold is-uppercase">Santé</p>
                <p className="dash-name">🍷 Alcoolémie</p>
              </div>
            </div>
          </div>
          <hr />

          <div className="box health-box" style={{ marginTop: '1rem' }}>

            {/* Genre + Poids */}
            <div style={{ display: 'flex', gap: '.625rem' }}>
              {(['male', 'female'] as Gender[]).map(g => (
                <button key={g}
                  style={gender === g
                    ? { flex: 1, background: 'var(--h-alcohol)', color: '#0f0e17', border: 'none', borderRadius: '10px', fontWeight: 700, padding: '.5rem' }
                    : { flex: 1, background: 'var(--surface-up)', border: '1px solid var(--border)', color: 'var(--txt-muted)', borderRadius: '10px', padding: '.5rem' }
                  }
                  onClick={() => setGender(g)}
                >
                  {g === 'male' ? '♂ Homme' : '♀ Femme'}
                </button>
              ))}
            </div>

            <div className="field">
              <label className="label health-label">Poids (kg)</label>
              <input className="input health-input" type="number" min={30} max={300}
                value={weight} onChange={e => setWeight(Number(e.target.value))} />
            </div>

            <div className="field">
              <label className="label health-label">Temps écoulé depuis le 1er verre (h)</label>
              <input className="input health-input" type="number" min={0} max={24} step={0.5}
                value={hours} onChange={e => setHours(Number(e.target.value))} />
            </div>

            {/* Sélecteur de boissons */}
            <div>
              <p className="health-subtitle" style={{ marginBottom: '.5rem' }}>Boissons consommées</p>
              <div style={{ display: 'flex', flexDirection: 'column', gap: '.375rem' }}>
                {(Object.keys(DRINK_DEF) as DrinkType[]).map(type => {
                  const def = DRINK_DEF[type];
                  const qty = drinks.find(d => d.type === type)?.quantity ?? 0;
                  return (
                    <div key={type} className="health-item" style={{ justifyContent: 'space-between' }}>
                      <span style={{ fontSize: '.875rem', color: 'var(--txt)' }}>
                        {def.emoji} {def.label}
                        <span style={{ fontSize: '.72rem', color: 'var(--txt-faint)', marginLeft: '.375rem' }}>
                          ({def.grams}g alc.)
                        </span>
                      </span>
                      <div style={{ display: 'flex', alignItems: 'center', gap: '.375rem' }}>
                        {qty > 0 && (
                          <button className="button alco-remove-btn" onClick={() => removeDrink(type)}>−</button>
                        )}
                        {qty > 0 && (
                          <span style={{ fontWeight: 700, color: 'var(--h-alcohol)', minWidth: '1.25rem', textAlign: 'center' }}>{qty}</span>
                        )}
                        <button className="button alco-add-btn" onClick={() => addDrink(type)}>+</button>
                      </div>
                    </div>
                  );
                })}
              </div>
            </div>

            {/* Résultat */}
            {drinks.length > 0 && (
              <div className={`alco-result ${level.cls}`}>
                <div className="alco-result-val">{taux.toFixed(2)} g/L</div>
                <p style={{ fontSize: '.8rem', fontWeight: 600, color: level.color, marginTop: '.25rem' }}>
                  {level.label}
                </p>
                {timeToZero > 0 && (
                  <p style={{ fontSize: '.75rem', color: 'var(--txt-muted)', marginTop: '.25rem' }}>
                    Retour à 0 estimé dans <strong>{timeToZero}h</strong>
                  </p>
                )}
              </div>
            )}

            {/* Disclaimer */}
            <p className="alco-disclaimer">
              ⚠️ Estimation basée sur la formule de Widmark. De nombreux facteurs
              (repas, médicaments, tolérance) influencent le taux réel.
              Ne prenez jamais le volant si vous avez consommé de l'alcool.
              Cet outil ne remplace pas un éthylomètre certifié.
            </p>

          </div>
        </div>
      </div>
    </section>
  );
};

export default AlcoolemiePage;