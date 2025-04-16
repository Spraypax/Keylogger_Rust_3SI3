use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::{Aead, AeadCore, OsRng};
use aes_gcm::aead::rand_core::RngCore;
use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::generic_array::typenum::U12;

use crate::modules::key_manager::get_or_create_key;

fn generate_nonce() -> Nonce<U12> {
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    Nonce::from_slice(&nonce_bytes).clone()
}

// Chiffre une chaîne de caractères avec AES-GCM
pub fn encrypt_log(data: &str) -> Vec<u8> {
    let key = get_or_create_key("passphrase"); // ⚠️ À adapter selon ton système de gestion de clé
    let cipher = Aes256Gcm::new(&key);
    let nonce = generate_nonce();

    match cipher.encrypt(&nonce, data.as_bytes()) {
        Ok(encrypted_data) => {
            let mut result = nonce.to_vec(); // clone du nonce en Vec<u8>
            result.extend_from_slice(&encrypted_data);
            result
        }
        Err(_) => panic!("Erreur lors du chiffrement !"),
    }
}
