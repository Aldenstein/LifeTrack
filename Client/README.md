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
VITE_API_URL=https://lifetrack.chocsathan.fr
```

Si tu utilises Windows PowerShell :

```powershell
$env:VITE_API_URL='https://lifetrack.chocsathan.fr'
```

> Note : ouvrir directement `https://lifetrack.chocsathan.fr` dans le navigateur peut renvoyer "non autorisé" si le backend n’expose pas de route publique à la racine. C’est normal : le front communique avec des endpoints comme `/auth/login` et `/users/me`.

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

Le front n’utilise plus de données de démonstration au démarrage. Il suffit de fournir l'URL de l'API via la variable d'environnement `VITE_API_URL` et d'exposer les endpoints suivants attendus par l'application.

- Configuration (exemple):

```bash
VITE_API_URL=http://localhost:8080
# ou dans Windows PowerShell
$env:VITE_API_URL='http://api.example.com'
```

- Auth (public):
	- `POST /auth/login`  — body: `{ "email": string, "password": string }` → response: `{ "token": string }`
	- `POST /auth/register` — body: `{ "email": string, "password": string, ... }` → response: `{ "token": string }`
	- `GET /users/me` — header: `Authorization: Bearer <token>` → response: profil utilisateur `{ "id": number, "email": string, "name": string, ... }`

- Finance (auth required, uses `Authorization: Bearer <token>`):
	- `GET /users/:userId/accounts` → liste des comptes
	- `POST /users/:userId/accounts` — créer compte, body: `{...}` → `{ "id": number }`
	- `GET /finance/types` → types de finance
	- `GET /users/:userId/transactions` → liste des transactions
	- `POST /users/:userId/transactions` — créer transaction, body: `{...}` → `{ "id": number }`
		- Note: le body peut contenir `date` (format ISO `YYYY-MM-DD`) pour fixer la date de la transaction. Si absent, la date est définie côté client lors de l'ajout.

	Notes supplémentaires:
	- Tâches (`/users/:userId/todos`): l'application envoie et reçoit le format suivant côté API — priorité est mappée en nombre (high=3, medium=2, low=1) et la date d'échéance est `due_date` (ISO `YYYY-MM-DD`).
	- Habitudes (`/users/:userId/habits`): l'application mappe les champs `title` / `frequency` du backend vers les champs UI (`name`, `frequency`).
	- `GET /users/:userId/planned-expenses` → dépenses planifiées
	- `POST /users/:userId/planned-expenses` — créer dépense planifiée, body: `{...}` → `{ "id": number }`

Remarques:
- L'application utilise un client HTTP commun (`src/renderer/src/services/api.ts`) qui lit `VITE_API_URL` à l'exécution. Assurez-vous que l'URL fournie ne se termine pas par un slash ou adaptez les endpoints en conséquence.
- Le token retourné par `/auth/login` ou `/auth/register` est stocké via `useUserStore` (persistant). Après authentification, l'application récupère automatiquement le profil via `/users/me`.

Après avoir configuré `VITE_API_URL`, relancez le mode développement:

```bash
npm run dev
```

Avec ces éléments en place, il ne reste plus qu'à fournir l'URL de votre API pour connecter l'application.
