# Projet_BDDA (Mini SGBD en RUST)

## Table des matières
- [Présentation](#présentation)
- [Installation](#installation)
- [Execution](#execution)
- [Utilisation](#utilisation)
- [Contributeurs](#contributeurs)

## Présentation 
Ce projet consiste à développer un mini Système de Gestion de Base de Données (SGBD) en **Rust**. Un SGBD est un logiciel permettant de gérer, manipuler et interroger des données de manière structurée. Ce projet met en œuvre certaines fonctionnalités essentielles d'un SGBD :

- Gestion des bases de données et des relations.
- Manipulation de tuples : insertion, suppression, et requêtes SELECT.
- Opérations relationnelles comme les sélections et les projections.

L'objectif est de maîtriser les bases des bases de données tout en exploitant la performance et la sécurité du langage **Rust**.

## Fonctionnalités
1. Commande [CREATE DATABASE](#create-database) : Créer une base de données.
2. Commande [CREATE TABLE](#create-table) : Créer une table dans la base de données.
3. Commande [SET DATABASE](#set-database) : Définir une base de données active.
4. Commande [LIST TABLES](#list-tables) : Afficher les tables d'une base active.
5. Commande [LIST DATABASES](#list-databases) : Afficher les bases de données.
6. Commande [DROP TABLE](#drop-table) : Supprimer une table.
7. Commande [DROP DATABASE](#drop-database) : Supprimer une base de données.
8. Commande [INSERT INTO](#insert-into) : Insérer des données dans une table.
9. Commande [SELECT](#select) : Récupérer des données depuis une table.

## Installation
### Prérequis
- **Rust** : Assurez-vous d'avoir Rust installé. Si ce n'est pas le cas, installez-le via [Rustup](https://www.rust-lang.org/tools/install).

```bash
# Vérifier l'installation de Rust
rustc --version
```

- **Cargo** : Le gestionnaire de projet Rust sera utilisé pour compiler et exécuter le code.

### Téléchargement du code source
Clonez le dépôt avec la commande suivante :

```bash
git clone https://github.com/JuriSOK/MiniSGBDR.git
cd MiniSGBDR
```

## Execution
Compilez et exécutez le projet avec **Cargo** :

```bash
cargo run
```

## Utilisation
Voici comment utiliser les commandes principales dans le Mini SGBD :

### Commande CREATE DATABASE
Création d'une nouvelle base de données.
```rust
CREATE DATABASE NomBDD;
```

### Commande CREATE TABLE
Création d'une table avec des colonnes et leurs types.
```rust
CREATE TABLE NomTable (NomCol_1:TypeCol_1, NomCol_2:TypeCol_2, ..., NomCol_N:TypeCol_N);
```

### Commande SET DATABASE
Sélection de la base de données courante.
```rust
SET DATABASE NomBDD;
```

### Commande LIST TABLES
Affiche toutes les tables de la base courante.
```rust
LIST TABLES;
```

### Commande LIST DATABASES
Affiche toutes les bases de données existantes.
```rust
LIST DATABASES;
```

### Commande DROP TABLE
Supprime une table spécifique.
```rust
DROP TABLE NomTable;
```

### Commande DROP DATABASE
Supprime une base de données.
```rust
DROP DATABASE NomBDD;
```

### Commande INSERT INTO
Insère un tuple (ligne) dans une table existante.
```rust
INSERT INTO NomTable VALUES (val1, val2, ..., valN);
```

### Commande SELECT
Récupère des données d'une table selon des conditions.
```rust
SELECT * FROM NomTable WHERE condition1 AND condition2;
```

## Contributeurs
- [SOK VIBOL ARNAUD](https://github.com/JuriSOK)
- [MOUSTACHE MATHIEU](https://github.com/whoismathieu)
- [LETACONNOUX AYMERIC](https://github.com/Shrek1515)
- [MEUNIER YOHANN](https://github.com/Ora-197)
