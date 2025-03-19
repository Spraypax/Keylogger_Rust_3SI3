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

### 🔹 Keylogging furtif  
- [`evdev`](https://docs.rs/evdev/latest/evdev/) - Capture des frappes clavier via `/dev/input/eventX` (Linux)  
- [`winapi`](https://docs.rs/winapi/latest/winapi/) - Hook système `SetWindowsHookEx` pour la capture clavier (Windows)  

### 🔹 Sécurité et chiffrement  
- [`aes-gcm`](https://docs.rs/aes-gcm/latest/aes_gcm/) - Chiffrement AES-GCM authentifié  
- [`rand`](https://docs.rs/rand/latest/rand/) - Génération sécurisée de clés aléatoires  
- [`serde`](https://docs.rs/serde/latest/serde/) + [`serde_json`](https://docs.rs/serde_json/latest/serde_json/) - Sérialisation et stockage des logs  

### 🔹 Exfiltration furtive des logs  
- [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) - Envoi furtif des logs via HTTP POST  
- [`tokio`](https://tokio.rs/) - Gestion asynchrone et multitâche  
- [`websocket`](https://docs.rs/websocket/latest/websocket/) - Envoi des logs en temps réel via WebSockets  

### 🔹 Persistance et gestion des fichiers  
- [`dirs`](https://docs.rs/dirs/latest/dirs/) - Gestion des chemins et stockage des logs dans des fichiers cachés  
- [`sysinfo`](https://docs.rs/sysinfo/latest/sysinfo/) - Récupération d'infos système (ex: masquer le processus)  


## 🔧 Installation et Utilisation  

### 1️⃣ Prérequis  
📌 Installation Rust et Cargo :  
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
