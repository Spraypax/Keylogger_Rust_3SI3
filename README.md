# 🛠 Keylogger en Rust  

## 📌 Description  
Ce projet est un **keylogger écrit en Rust** qui capture les frappes du clavier et les enregistre de manière sécurisée. Il est conçu à des **fins éducatives** pour explorer la capture d'événements clavier, la journalisation et la sécurité des données en Rust.  

⚠️ **Note** : Ce projet doit être utilisé uniquement dans un cadre légal et éthique. Toute utilisation malveillante est strictement interdite.  

## 🚀 Fonctionnalités  
✔️ Capture des frappes clavier en temps réel (Windows, Linux, macOS)  
✔️ Enregistrement des données localement dans un fichier chiffré  
✔️ Option pour envoyer les logs à un serveur distant  
✔️ Exécution en arrière-plan (mode furtif)  
✔️ Interface CLI pour gérer le keylogger  
✔️ Démarrage automatique au boot (optionnel)  

## 📦 Technologies utilisées  
- **Rust** (langage principal)  
- [`rdev`](https://github.com/Narsil/rdev) - Pour la capture des frappes clavier  
- [`tokio`](https://tokio.rs/) - Gestion asynchrone des tâches  
- [`aes-gcm`](https://docs.rs/aes-gcm) - Chiffrement des logs  
- [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) - Envoi des logs à un serveur  

## 🔧 Installation et Utilisation  

### 1️⃣ Prérequis  
📌 Installation Rust et Cargo :  
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
