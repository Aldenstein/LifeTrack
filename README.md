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
│   ├── auth.rs
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

Ce dossier contient l’ensemble du code source Rust de l’application. Il regroupe le point d’entrée, la configuration, les accès à la base de données, les modèles, les routes et les modules utilitaires nécessaires au fonctionnement du backend.

###### `src/main.rs`
Point d’entrée principal de l’application.  
Ce fichier est exécuté au démarrage avec `cargo run` et sert à initialiser la configuration, les connexions partagées, l’état global et le lancement du serveur ou de la logique principale.

###### `src/api.rs`
Contient la logique applicative exposée par l’API.  
On y place généralement les handlers, les traitements métier liés aux requêtes entrantes et les fonctions appelées par les routes HTTP.

###### `src/config.rs`
Centralise la configuration du projet.  
Ce module sert à charger les variables d’environnement et les paramètres applicatifs, par exemple `DATABASE_URL`, l’adresse du serveur ou d’autres options d’exécution.

###### `src/db.rs`
Gère l’accès à la base de données.  
On y place la création du pool de connexions, les helpers d’accès SQL et les fonctions de persistance nécessaires au backend.

###### `src/errors.rs`
Définit les erreurs applicatives du projet.  
Ce module permet d’uniformiser la gestion des erreurs, de centraliser les messages et de simplifier leur propagation dans les différentes couches de l’application.

###### `src/models.rs`
Contient les structures de données du projet.  
On y définit les modèles métier, les structures échangées avec la base de données et les objets sérialisés ou désérialisés en JSON pour l’API.

###### `src/routes.rs`
Déclare et organise les routes de l’application.  
Ce fichier permet de regrouper les endpoints HTTP, d’associer les chemins aux handlers et de garder une structure claire côté serveur.

###### `src/sql_to_json.rs`
Module spécialisé dans la transformation de données SQL vers JSON.  
Il peut contenir la logique qui récupère des données depuis la base ou depuis des requêtes SQL, puis les convertit dans un format JSON exploitable par l’API ou la webapp.

###### `src/utils.rs`
Contient les fonctions utilitaires partagées entre plusieurs modules.  
On y place les helpers génériques réutilisables, tant qu’ils ne relèvent pas d’un domaine métier ou d’un module spécifique.

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
