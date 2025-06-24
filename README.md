
# 🎯 Keylogger Rust — Projet 3SI3

> ✅ **Projet finalisé : Keylogger multiplateforme en Rust avec persistance, exfiltration via serveur C2 et gestion de logs.**

## 📌 Description
Ce projet est un keylogger écrit en Rust qui capture les frappes du clavier et les enregistre de manière sécurisée. Il est conçu à des fins éducatives pour explorer la capture d'événements clavier, la journalisation et la sécurité des données en Rust. (sur Linux)

## ⚠️ Note : Ce projet doit être utilisé uniquement dans un cadre légal et éthique. Toute utilisation malveillante est strictement interdite.

## 📦 Technologies utilisées

Rust (langage principal)

## 📂 Structure du projet

```
Keylogger_Rust_3SI3/
├── Victime/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── key.bin
│   └── src/
│       ├── main.rs
│       ├── config.rs
│       └── modules/
│           ├── logger.rs
│           ├── network.rs
│           ├── persistence.rs
│           └── mod.rs
│
├── Attaquant/
│   ├── Cargo.toml
│   ├── Cargo.lock
│   ├── key.bin
│   └── src/
│       ├── main.rs
│       ├── Logs/
│       │   ├── log.log
│       │   ├── logC2.log
│       └── modules/
│           ├── key_manager.rs
│           ├── logger.rs
│           ├── network.rs
│           └── mod.rs
```

## ✅ Prérequis

### 📦 Packages système minimum

```bash
sudo apt update && sudo apt install -y \    xterm \    pkg-config \    libssl-dev \    libevdev-dev \    build-essential \    cargo \    rustup

rustup default stable
```

## ⚙️ Compilation

```bash
# Côté Victime
cd /mnt/hgfs/Keylogger_Rust_3SI3/Victime
cargo build --release

# Côté Attaquant
cd ~/Keylogger_Rust_3SI3/Attaquant
cargo build --release
```

## 🚀 Utilisation

### 🕵️‍♂️ 1️⃣ Lancer le menu Attaquant

```bash
sudo ./target/release/attacker-keylogger-rust start
```

- Menu CLI interactif pour :
  1. Serveur C2
  2. Mode local
  3. Supprimer les logs
  4. Quitter

1. Les frappes reçues sont enregistrées dans `src/Logs/logC2.log`.
2. Les frappes reçues sont enregistrées dans `src/Logs/log.log`.

### 🖥️ 2️⃣ Lancer le Keylogger Victime

```bash
sudo ./target/release/Victim-keylogger-rust
```

- Détecte `/dev/input/eventX`, layout clavier, et se connecte automatiquement au C2.

### 📄 3️⃣ Logs

| Mode | Fichier log |
|------|--------------|
| **Mode Local (Attaquant)** | `src/Logs/log.log` |
| **Mode Serveur C2 (Attaquant)** | `src/Logs/logC2.log` |

## 📡 Connexion réseau

- Le port du Serveur C2 doit être ouvert.
- Reconnexion automatique côté Victime (essaie toutes les 5 secondes).

## 🔒 Persistance

- Gérée par `persistence.rs` côté Victime.

## ✅ Bonnes pratiques

- Toujours exécuter avec `sudo`.
- Compiler en `--release` pour optimiser.
- Supprimer les logs régulièrement.

## 🧹 Nettoyer les logs

Depuis le menu CLI : `3) Supprimer les logs`
Puis choisir le fichier à nettoyer.

## ✅ Résumé

| 📂 Côté | Commande |
|---------|-----------|
| Build Victime | `cargo build --release` |
| Lancer Victime | `sudo ./target/release/Victim-keylogger-rust` |
| Build Attaquant | `cargo build --release` |
| Lancer Attaquant | `sudo ./target/release/attacker-keylogger-rust start` |


## ✅ Auteur

- DUPONT Lélian & KLEIN Dylan
