# Documentation du projet

   LifeTrack à pour objectif d'aider les personnes à rendre compte de leurs habitudes, qu'elles soient positives ou négatives. Ce qui permet de faire un suivi régulier du quotidien des utilisateurs.

### Base de données et API

#### Arborescence du projet LifetrackDB

##### Vue d'ensemble

Ce projet est organisé autour d'une application Rust, de scripts SQL et des fichiers standards de configuration Cargo.

```text
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── requirement.txt
├── SQL
│   ├── requests.sql
│   ├── selftrack_create.sql
│   └── selftrack_db_create.sql
├── src
│   ├── api.rs
│   ├── config.rs
│   ├── db.rs
│   ├── errors.rs
│   ├── main.rs
│   ├── models.rs
│   ├── routes.rs
│   ├── sql_to_json.rs
│   └── utils.rs
└── target
```

##### Fichiers racine

###### `Cargo.toml`
Fichier principal de configuration du projet Rust.  
Il contient le nom du package, la version, l’édition Rust utilisée et la liste des dépendances nécessaires au projet.

###### `Cargo.lock`
Fichier généré par Cargo qui fige précisément les versions des dépendances installées.  
Il garantit que le projet se compile avec les mêmes versions d’une machine à l’autre.

###### `README.md`
Documentation principale du projet.  
Il doit présenter l’objectif du projet, les prérequis, les commandes d’installation et les étapes de lancement.

###### `requirement.txt`
Fichier auxiliaire présent à la racine.  
Son rôle dépend du projet : il peut servir de pense-bête, de dépendances externes non Rust, ou d’héritage d’une ancienne structure.

##### Dossier `SQL`

Ce dossier contient les scripts SQL utilisés par le projet.

###### `SQL/requests.sql`
Regroupe les requêtes SQL utiles à l’application.  
Il peut servir de référence ou contenir des requêtes testées avant intégration dans le code Rust.

###### `SQL/selftrack_create.sql`
Script de création de tables ou d’objets SQL liés au cœur fonctionnel du projet.  
Il sert à initialiser une partie du schéma de données.

###### `SQL/selftrack_db_create.sql`
Script de création globale de la base de données.  
Il peut contenir la création de la base, du schéma principal ou des éléments structurants initiaux.

##### Dossier `src`

Ce dossier contient tout le code source Rust du projet.

###### `src/main.rs`
Point d’entrée de l’application.  
C’est le fichier exécuté au démarrage avec `cargo run`. Il initialise l’application, charge la configuration et lance la logique principale.

###### `src/api.rs`
Contient la logique de l’API.  
On y place généralement les handlers, les appels réseau ou les fonctions qui exposent les fonctionnalités vers l’extérieur.

###### `src/config.rs`
Centralise la configuration du projet.  
Ce fichier sert souvent à lire les variables d’environnement comme `DATABASE_URL`, `API_HOST` ou `API_PORT`.

###### `src/db.rs`
Gère la connexion à la base de données.  
On y place la création du pool de connexion, les helpers d’accès SQL et la logique bas niveau de persistance.

###### `src/errors.rs`
Définit les erreurs applicatives.  
Il permet d’uniformiser la gestion des erreurs pour éviter de disperser les messages et les types d’erreurs dans tout le projet.

###### `src/models.rs`
Contient les structures de données du projet.  
On y définit les modèles métier, les structs échangées avec la base de données ou les objets sérialisés en JSON.

###### `src/routes.rs`
Déclare les routes de l’application.  
Ce fichier est utile si le projet expose une API HTTP et permet d’organiser les endpoints proprement.

###### `src/sql_to_json.rs`
Module spécialisé dans la transformation de données SQL vers JSON.  
Il peut contenir la logique qui exécute une requête puis convertit le résultat dans un format JSON exploitable.

###### `src/utils.rs`
Contient les fonctions utilitaires partagées.  
On y place les helpers génériques réutilisés dans plusieurs modules, tant qu’ils ne méritent pas un module métier dédié.

##### Dossier `target`

###### `target/`
Dossier généré automatiquement par Cargo lors de la compilation.  
Il contient les fichiers temporaires, les builds intermédiaires et les exécutables compilés.

> Ce dossier ne doit généralement pas être versionné dans Git.  
> Il est recommandé de l’ajouter dans `.gitignore`.

##### Résumé d’utilisation

- `Cargo.toml` : configuration Rust du projet
- `src/` : code source principal
- `SQL/` : scripts de base de données
- `target/` : artefacts de compilation
- `README.md` : documentation du projet
