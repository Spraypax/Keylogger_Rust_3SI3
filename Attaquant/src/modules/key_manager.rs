use aes_gcm::{Aes256Gcm, Key, KeyInit};
use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::aead::rand_core::RngCore;
use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

const KEY_FILE: &str = "key.bin";
const SALT_SIZE: usize = 16;  // Taille du sel pour PBKDF2
const NONCE_SIZE: usize = 12; // Taille du nonce pour AES-GCM
const PBKDF2_ITERATIONS: u32 = 100_000;

/// Génère une nouvelle clé AES-256
fn generate_key() -> Key<Aes256Gcm> {
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);
    Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
}

/// Dérive une clé AES-256 à partir d'une passphrase
fn derive_key_from_passphrase(passphrase: &str, salt: &[u8]) -> Key<Aes256Gcm> {
    let mut key_bytes = [0u8; 32];
    pbkdf2_hmac::<Sha256>(
        passphrase.as_bytes(),
        salt,
        PBKDF2_ITERATIONS,
        &mut key_bytes,
    );
    Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
}

/// Chiffre la clé AES-256 avec la passphrase et la stocke dans key.bin
fn encrypt_and_save_key(key: &Key<Aes256Gcm>, passphrase: &str) -> std::io::Result<()> {
    let mut salt = [0u8; SALT_SIZE];
    let mut nonce = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    let derived_key = derive_key_from_passphrase(passphrase, &salt);
    let cipher = Aes256Gcm::new(&derived_key);

    let encrypted_key = cipher.encrypt(&nonce.into(), key.as_slice()).expect("Chiffrement échoué");

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(KEY_FILE)?;
    
    file.write_all(&salt)?;
    file.write_all(&nonce)?;
    file.write_all(&encrypted_key)?;
    file.sync_all()?;
    Ok(())
}

/// Déchiffre la clé AES-256 depuis key.bin avec la passphrase
fn decrypt_key(passphrase: &str) -> std::io::Result<Key<Aes256Gcm>> {
    let mut file = File::open(KEY_FILE)?;
    let mut salt = [0u8; SALT_SIZE];
    let mut nonce = [0u8; NONCE_SIZE];

    file.read_exact(&mut salt)?;
    file.read_exact(&mut nonce)?;

    let derived_key = derive_key_from_passphrase(passphrase, &salt);
    let cipher = Aes256Gcm::new(&derived_key);

    let mut encrypted_key = Vec::new();
    file.read_to_end(&mut encrypted_key)?;

    let decrypted_key = cipher.decrypt(&nonce.into(), encrypted_key.as_ref())
        .expect("Erreur de déchiffrement");

    Ok(Key::<Aes256Gcm>::from_slice(&decrypted_key).clone())
}

/// Vérifie si la clé existe, sinon la génère et la sauvegarde chiffrée
pub fn get_or_create_key(passphrase: &str) -> Key<Aes256Gcm> {
    if Path::new(KEY_FILE).exists() {
        match decrypt_key(passphrase) {
            Ok(key) => key,
            Err(_) => {
                println!("Erreur de lecture de la clé, régénération...");
                let key = generate_key();
                encrypt_and_save_key(&key, passphrase).expect("Impossible de sauvegarder la clé !");
                key
            }
        }
    } else {
        println!("Aucune clé trouvée, génération d'une nouvelle clé...");
        let key = generate_key();
        encrypt_and_save_key(&key, passphrase).expect("Impossible de sauvegarder la clé !");
        key
    }
}
