# my-app

Application Electron + React + TypeScript pour le suivi de la vie quotidienne: finances, habitudes, tâches et santé.

## Démarrage

### Installation

```bash
npm install
```

### Configuration API

Le front attend une API accessible via `VITE_API_URL`.

```bash
VITE_API_URL=http://localhost:8080
```

### Lancer le projet

```bash
npm run dev
```

### Vérifications

```bash
npm run lint
npm run typecheck
```

## Rôle des fichiers

### Racine

- `dev-app-update.yml` : configuration de mise à jour pour le mode dev.
- `electron-builder.yml` : packaging desktop.
- `electron.vite.config.ts` : build Electron/Vite.
- `eslint.config.mjs` : règles de lint.
- `package.json` : scripts et dépendances.
- `tsconfig.json` : configuration TypeScript racine.
- `tsconfig.node.json` : TypeScript pour le main process.
- `tsconfig.web.json` : TypeScript pour le renderer.
- `README.md` : documentation du projet.

### `build/`

- `build/entitlements.mac.plist` : permissions macOS.
- `build/resources/` : ressources de packaging.

### `src/main/`

- `src/main/index.ts` : point d’entrée Electron côté main process.

### `src/preload/`

- `src/preload/index.ts` : exposition sécurisée des APIs Electron au renderer.
- `src/preload/index.d.ts` : types du bridge preload.

### `src/renderer/index.html`

- Template HTML principal chargé par Vite.

### `src/renderer/src/main.tsx`

- Monte React dans la page et lance l’application sans seed local.

### `src/renderer/src/App.tsx`

- Définit le router principal et les routes protégées.

### `src/renderer/src/components/`

- `BackButton.tsx` : bouton de retour réutilisable.

#### `components/auth/`

- `AuthCard.tsx` : conteneur visuel de connexion/inscription.
- `LoginForm.tsx` : connexion via `authService` et `useAuth`.
- `RegisterForm.tsx` : inscription via `authService` et `useAuth`.

#### `components/finance/`

- `CompteCard.tsx` : carte d’un compte.
- `CompteDetail.tsx` : détail d’un compte et de ses opérations.
- `CompteModal.tsx` : création d’un compte via l’API.
- `PlannedExpenseCard.tsx` : carte d’une dépense planifiée.
- `PlannedExpenseModal.tsx` : création d’une dépense planifiée via l’API.
- `TransactionCard.tsx` : carte d’une transaction.
- `TransactionModal.tsx` : création d’une transaction via l’API.

#### `components/habits/`

- `HabitCheckCard.tsx` : affichage d’une habitude à validation simple.
- `HabitCounterCard.tsx` : affichage d’une habitude à compteur.
- `HabitModal.tsx` : création ou édition d’une habitude.

#### `components/health/`

- `HealthHome.tsx` : page d’accueil santé.
- `HealthTuile.tsx` : tuile réutilisable.
- `alcoolemie/AlcoolemiePage.tsx` : suivi alcoolémie.
- `coherence/CoherencePage.tsx` : suivi cohérence.
- `humeur/HumeurPage.tsx` : suivi humeur.
- `hydratation/HydratationPage.tsx` : suivi hydratation.
- `ponderaux/PonderauxPage.tsx` : suivi du poids.
- `repas/RepasPage.tsx` : suivi repas.
- `sobriete/SobrieteHome.tsx` : suivi sobriété.
- `sommeil/SommeilPage.tsx` : suivi sommeil.
- `sport/SportPage.tsx` : suivi sport.

#### `components/home/`

- `GaugeChart.tsx` : jauge de progression.
- `HomePanel.tsx` : tableau de bord principal.
- `Tiles.tsx` : tuiles d’accès rapide.

#### `components/todos/`

- `TodoCard.tsx` : carte d’une tâche.
- `TodoModal.tsx` : création ou édition d’une tâche.

### `src/renderer/src/hooks/`

- `useAuth.ts` : connexion, inscription et déconnexion avec l’API.
- `useBootstrap.ts` : restauration de session et chargement des données au démarrage.
- `useLoadFinance.ts` : chargement des données finance depuis l’API.

### `src/renderer/src/pages/`

- `AuthPage.tsx` : page publique de connexion et d’inscription.
- `FinancePage.tsx` : page finances.
- `HabitsPage.tsx` : page habitudes.
- `HomePage.tsx` : page d’accueil.
- `TodoPage.tsx` : page tâches.

### `src/renderer/src/services/`

- `api.ts` : client HTTP commun avec gestion du token.
- `authService.ts` : endpoints d’authentification.
- `financeService.ts` : endpoints finance.
- `userService.ts` : endpoints profil utilisateur.

### `src/renderer/src/store/`

- `index.ts` : réexport des stores.
- `useAuthStore.ts` : ancien store local d’auth, conservé pour compatibilité si besoin.
- `useFinanceStore.ts` : état des finances.
- `useHabitStore.ts` : état des habitudes.
- `useTodoStore.ts` : état des tâches.
- `userStore.ts` : token et profil utilisés par le flux API.

### `src/renderer/src/styles/`

- `auth.css` : styles auth.
- `back-button.css` : styles du bouton retour.
- `finance.css` : styles finance.
- `financ.css` : feuille legacy à vérifier.
- `habits.css` : styles habitudes.
- `health.css` : styles santé.
- `home.css` : styles accueil.
- `todos.css` : styles tâches.

### `src/renderer/src/types/`

- `api.ts` : contrats des réponses et requêtes API.
- `finance.ts` : types métier finance.
- `habit.ts` : types métier habitudes.
- `health.ts` : types métier santé.
- `todo.ts` : types métier tâches.
- `user.ts` : type utilisateur local.
- `index.ts` : réexports des types.

### `src/renderer/src/utils/`

- `facture.ts` : utilitaires liés aux calculs finance.

## Intégration API

Le front n’utilise plus de données de démonstration au démarrage. Pour connecter le backend, il suffit de fournir les endpoints attendus par `authService` et `financeService`, puis de renseigner `VITE_API_URL`.
