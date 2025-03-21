use aes_gcm::{Aes256Gcm, Key, KeyInit};
use rand::Rng;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use zeroize::Zeroize;

const KEY_FILE: &str = "key.bin";

// Génère une nouvelle clé AES-256
fn generate_key() -> Key<Aes256Gcm> {
    Key::<Aes256Gcm>::generate(&mut rand::thread_rng())
}

// Sauvegarde la clé dans un fichier sécurisé
fn save_key_to_file(key: &Key<Aes256Gcm>) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(KEY_FILE)?;
    
    file.write_all(&key[..])?;
    file.sync_all()?;  
    Ok(())
}

// Charge la clé depuis le fichier sécurisé
fn load_key_from_file() -> std::io::Result<Key<Aes256Gcm>> {
    let mut file = File::open(KEY_FILE)?;
    let mut key_bytes = [0u8; 32]; 
    file.read_exact(&mut key_bytes)?;

    Ok(Key::<Aes256Gcm>::from_slice(&key_bytes).clone())
}

// Vérifie si la clé existe, sinon la génère et la sauvegarde
pub fn get_or_create_key() -> Key<Aes256Gcm> {
    if Path::new(KEY_FILE).exists() {
        match load_key_from_file() {
            Ok(key) => key,
            Err(_) => {
                println!("Erreur de lecture de la clé, régénération...");
                let key = generate_key();
                save_key_to_file(&key).expect("Impossible de sauvegarder la clé !");
                key
            }
        }
    } else {
        println!("Aucune clé trouvée, génération d'une nouvelle clé...");
        let key = generate_key();
        save_key_to_file(&key).expect("Impossible de sauvegarder la clé !");
        key
    }
}
