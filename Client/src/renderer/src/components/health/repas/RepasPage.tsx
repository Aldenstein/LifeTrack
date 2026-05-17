// ============================================================
// RepasPage.tsx  Suivi des repas et calories consommées
// Saisie simple : type de repas, nom, calories, note
// Données stockées en localStorage sous "health_meals"
// ============================================================

import React, { useEffect, useMemo, useState } from 'react';
import BackButton from '../../BackButton';
import { MealEntry, MealType } from '../../../types/health';
import { healthService } from '@/services/healthService';
import { useUserStore } from '@/store/userStore';
import '../../../styles/health.css';

const TODAY = new Date().toISOString().slice(0, 10);

const MEAL_TYPES: { value: MealType; label: string; emoji: string }[] = [
  { value: 'breakfast', label: 'Petit-déjeuner', emoji: '🍳' },
  { value: 'lunch', label: 'Déjeuner', emoji: '🍽️' },
  { value: 'dinner', label: 'Dîner', emoji: '🌙' },
  { value: 'snack', label: 'Collation', emoji: '🍌' },
];

function typeLabel(type: MealType): string {
  return MEAL_TYPES.find(t => t.value === type)?.label ?? type;
}

const RepasPage: React.FC = () => {
  const userId = useUserStore(s => s.profile?.id) ?? 0;
  const [meals, setMeals] = useState<MealEntry[]>([]);

  const [mealType, setMealType] = useState<MealType>('lunch');
  const [name, setName] = useState('');
  const [calories, setCalories] = useState(450);
  const [note, setNote] = useState('');

  useEffect(() => {
    void (async () => {
      try {
        setMeals(await healthService.getMealEntries(userId));
      } catch {
        setMeals([]);
      }
    })();
  }, [userId]);

  const todayMeals = useMemo(
    () => meals.filter(m => m.date === TODAY).sort((a, b) => b.id.localeCompare(a.id)),
    [meals]
  );

  const todayTotal = todayMeals.reduce((sum, meal) => sum + meal.calories, 0);
  const weekTotal = useMemo(() => {
    const days = Array.from({ length: 7 }, (_, i) => {
      const d = new Date();
      d.setDate(d.getDate() - i);
      return d.toISOString().slice(0, 10);
    });
    return meals.filter(m => days.includes(m.date)).reduce((sum, meal) => sum + meal.calories, 0);
  }, [meals]);

  const handleSave = async () => {
    if (!name.trim() || calories <= 0) return;

    const entry: MealEntry = {
      id: Date.now().toString(),
      date: TODAY,
      mealType,
      name: name.trim(),
      calories: Math.round(calories),
      note: note.trim() || undefined,
    };

    try {
      const created = await healthService.createMealEntry(userId, entry);
      setMeals(prev => [created, ...prev]);
    } catch {
      return;
    }
    setName('');
    setCalories(450);
    setNote('');
    setMealType('lunch');
  };

  const handleDelete = async (id: string) => {
    try {
      await healthService.deleteMealEntry(userId, id);
      setMeals(prev => prev.filter(m => m.id !== id));
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
                <p className="dash-name">🍽️ Repas</p>
              </div>
            </div>
          </div>
          <hr />

          <div className="box health-box" style={{ marginTop: '1rem' }}>
            <div className="columns is-mobile" style={{ width: '100%', margin: 0 }}>
              <div className="column" style={{ padding: '.25rem' }}>
                <div className="pond-result" style={{ padding: '1rem' }}>
                  <div className="pond-result-val" style={{ color: 'var(--h-mood)', fontSize: '1.9rem' }}>{todayTotal}</div>
                  <div className="pond-result-label">Calories aujourd'hui</div>
                </div>
              </div>
              <div className="column" style={{ padding: '.25rem' }}>
                <div className="pond-result" style={{ padding: '1rem' }}>
                  <div className="pond-result-val" style={{ color: 'var(--h-weight)', fontSize: '1.9rem' }}>{weekTotal}</div>
                  <div className="pond-result-label">Calories sur 7 jours</div>
                </div>
              </div>
            </div>

            <div className="field">
              <label className="label health-label">Type de repas</label>
              <div className="mood-picker" style={{ justifyContent: 'space-between', flexWrap: 'wrap' }}>
                {MEAL_TYPES.map(t => (
                  <button
                    key={t.value}
                    className={`button mood-btn${mealType === t.value ? ' is-active' : ''}`}
                    style={mealType === t.value ? { '--tc': 'var(--h-mood)' } as React.CSSProperties : {}} 
                    onClick={() => setMealType(t.value)}
                    type="button"
                  >
                    {t.emoji}
                  </button>
                ))}
              </div>
              <p className="health-subtitle" style={{ marginTop: '.5rem' }}>{typeLabel(mealType)}</p>
            </div>

            <div className="field">
              <label className="label health-label">Nom du repas</label>
              <input
                className="input health-input"
                type="text"
                value={name}
                placeholder="Ex. Poulet riz légumes"
                onChange={e => setName(e.target.value)}
              />
            </div>

            <div className="columns is-mobile" style={{ width: '100%', margin: 0 }}>
              <div className="column" style={{ padding: '.25rem' }}>
                <div className="field">
                  <label className="label health-label">Calories</label>
                  <input
                    className="input health-input"
                    type="number"
                    min={0}
                    step={10}
                    value={calories}
                    onChange={e => setCalories(Number(e.target.value))}
                  />
                </div>
              </div>
            </div>

            <div className="field">
              <label className="label health-label">Note (optionnel)</label>
              <textarea
                className="textarea health-input"
                rows={2}
                value={note}
                placeholder="Restaurant, portion, macros..."
                onChange={e => setNote(e.target.value)}
              />
            </div>

            <button
              className="button health-btn-submit"
              onClick={handleSave}
              disabled={!name.trim() || calories <= 0}
              style={{ '--tc': 'var(--h-mood)', '--tg': 'var(--h-mood-glow)' } as React.CSSProperties}
            >
              Ajouter le repas
            </button>

            <hr style={{ margin: 0 }} />

            <div>
              <p className="health-subtitle" style={{ marginBottom: '.75rem' }}>Repas enregistrés</p>
              {todayMeals.length === 0 ? (
                <div className="notification health-empty">Aucun repas enregistré aujourd'hui.</div>
              ) : (
                <div style={{ display: 'flex', flexDirection: 'column', gap: '.625rem' }}>
                  {todayMeals.map(meal => (
                    <div key={meal.id} className="health-item" style={{ justifyContent: 'space-between', alignItems: 'flex-start' }}>
                      <div className="health-item-info">
                        <div className="health-item-title">
                          {MEAL_TYPES.find(t => t.value === meal.mealType)?.emoji} {meal.name}
                        </div>
                        <div className="health-item-meta">
                          {typeLabel(meal.mealType)}{meal.note ? ` • ${meal.note}` : ''}
                        </div>
                      </div>
                      <div style={{ display: 'flex', alignItems: 'center', gap: '.5rem' }}>
                        <span style={{ fontWeight: 700, color: 'var(--h-mood)', whiteSpace: 'nowrap' }}>{meal.calories} kcal</span>
                        <button className="button alco-remove-btn" onClick={() => handleDelete(meal.id)}>−</button>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default RepasPage;
