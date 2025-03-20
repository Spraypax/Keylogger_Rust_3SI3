use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::{Aead, AeadCore, OsRng};
use crate::modules::key_manager::get_or_create_key;

// Génère un nonce sécurisé de 12 octets
fn generate_nonce() -> Nonce<Aes256Gcm> {
    Aes256Gcm::generate_nonce(&mut OsRng)
}

// Chiffre une chaîne de caractères avec AES-GCM
pub fn encrypt_log(data: &str) -> Vec<u8> {
    let key = get_or_create_key(); // Charge la clé stockée
    let cipher = Aes256Gcm::new(&key);
    let nonce = generate_nonce();

    match cipher.encrypt(&nonce, data.as_bytes()) {
        Ok(encrypted_data) => {
            let mut result = nonce.to_vec();
            result.extend_from_slice(&encrypted_data);
            result
        }
        Err(_) => panic!("Erreur lors du chiffrement !"),
    }
}
