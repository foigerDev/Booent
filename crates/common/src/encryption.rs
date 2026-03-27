use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use error_stack::{Report, ResultExt};
use rand::RngCore;
use secrecy::{ExposeSecret, Secret};
use sha2::{Sha256, Digest};

use crate::errors;

const NONCE_LENGTH: usize = 12;

/// Derive a 32-byte key from a string using SHA-256
fn derive_key(key_string: &Secret<String>) -> Result<Vec<u8>, Report<errors::EncryptionErrorTypes>> {
    let mut hasher = Sha256::new();
    hasher.update(key_string.expose_secret().as_bytes());
    let hashed = hasher.finalize();
    Ok(hashed.to_vec())
}

pub fn encrypt_string(
    plaintext: &str,
    encryption_key: &Secret<String>,
) -> Result<String, Report<errors::EncryptionErrorTypes>> {
    let key_bytes = derive_key(encryption_key)?;

    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .change_context(errors::EncryptionErrorTypes::EncryptionFailed)?;

    let mut nonce_bytes = [0u8; NONCE_LENGTH];
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from(nonce_bytes);

    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|_| Report::new(errors::EncryptionErrorTypes::EncryptionFailed))
        .attach_printable("AES-GCM encryption failed")?;

    let mut combined = nonce_bytes.to_vec();
    combined.extend(ciphertext);

    Ok(URL_SAFE_NO_PAD.encode(combined))
}

pub fn decrypt_string(
    encrypted_value: &str,
    encryption_key: &Secret<String>,
) -> Result<Secret<String>, Report<errors::EncryptionErrorTypes>> {
    let decoded = URL_SAFE_NO_PAD
        .decode(encrypted_value)
        .change_context(errors::EncryptionErrorTypes::DecryptionFailed)?;

    let (nonce_bytes, ciphertext) = decoded.split_at(NONCE_LENGTH);

    let key_bytes = derive_key(encryption_key)?;

    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .change_context(errors::EncryptionErrorTypes::DecryptionFailed)?;

    let nonce_array: [u8; NONCE_LENGTH] = nonce_bytes
    .try_into()
    .map_err(|_| Report::new(errors::EncryptionErrorTypes::DecryptionFailed)
        .attach_printable("Invalid nonce length"))?;
    let nonce = Nonce::from(nonce_array);

    let decrypted = cipher
        .decrypt(&nonce, ciphertext)
        .map_err(|_| Report::new(errors::EncryptionErrorTypes::DecryptionFailed))
        .attach_printable("AES-GCM encryption failed")?;

    let plaintext = String::from_utf8(decrypted)
        .change_context(errors::EncryptionErrorTypes::DecryptionFailed)?;

    Ok(Secret::new(plaintext))
}
