use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::{Aead, AeadCore};
use crate::modules::key_manager::get_or_create_key;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Fonction pour déchiffrer un bloc AES-GCM
fn decrypt_log(data: &[u8]) -> Option<String> {
    if data.len() < 12 {
        return None;
    }

    let (nonce_bytes, encrypted_data) = data.split_at(12); // nonce = 12 octets
    let key = get_or_create_key();
    let cipher = Aes256Gcm::new(&key);
    let nonce = Nonce::from_slice(nonce_bytes);

    match cipher.decrypt(nonce, encrypted_data) {
        Ok(decrypted) => String::from_utf8(decrypted).ok(),
        Err(_) => None,
    }
}

// Fonction principale de lecture du fichier logs.enc
pub fn read_encrypted_logs(path: &str) {
    let file = File::open(path).expect("Impossible d'ouvrir le fichier de logs !");
    let reader = BufReader::new(file);

    // Pour chaque ligne (bloc chiffré), on tente un déchiffrement
    for line in reader.lines() {
        if let Ok(hex_line) = line {
            if let Ok(bytes) = hex::decode(hex_line.trim()) {
                if let Some(text) = decrypt_log(&bytes) {
                    println!("→ {}", text);
                } else {
                    println!("!! Bloc non déchiffrable");
                }
            }
        }
    }
}
