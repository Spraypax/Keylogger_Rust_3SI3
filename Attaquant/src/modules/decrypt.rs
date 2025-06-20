use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn read_plaintext_logs() {
    let log_path = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("Logs")
        .join("log.txt");

    let file = File::open(&log_path)
        .unwrap_or_else(|e| panic!("Impossible d'ouvrir le fichier de logs !: {:?}: {}", log_path, e));
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(content) => println!("→ {}", content),
            Err(_) => eprintln!("!! Erreur de lecture du fichier"),
        }
    }
}
