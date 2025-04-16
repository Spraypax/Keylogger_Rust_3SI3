use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use crate::modules::key_manager::get_or_create_key;
use std::fs::File;
use std::io::{BufRead, BufReader};
use hex;
use dirs;

// Déchiffre un bloc AES-GCM
fn decrypt_log(data: &[u8], passphrase: &str) -> Option<String> {
    if data.len() < 12 {
        return None;
    }

    let (nonce_bytes, encrypted_data) = data.split_at(12);
    let key = get_or_create_key(passphrase);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(nonce_bytes);

    cipher.decrypt(nonce, encrypted_data).ok()
        .and_then(|decrypted| String::from_utf8(decrypted).ok())
}

// Lit le fichier logs.enc et tente de déchiffrer ligne par ligne
pub fn read_encrypted_logs(_path: &str, passphrase: &str) {
    let path = dirs::data_local_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap())
        .join("logs.enc");

    let file = File::open(path).expect("Impossible d'ouvrir le fichier de logs !");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(hex_line) => {
                match hex::decode(hex_line.trim()) {
                    Ok(bytes) => {
                        match decrypt_log(&bytes, passphrase) {
                            Some(text) => println!("→ {}", text),
                            None => eprintln!("!! Bloc non déchiffrable"),
                        }
                    }
                    Err(_) => eprintln!("!! Ligne illisible"),
                }
            }
            Err(_) => eprintln!("!! Erreur de lecture du fichier"),
        }
    }
}
