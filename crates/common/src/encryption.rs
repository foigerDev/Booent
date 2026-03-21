use aes_gcm::aead::{Aead, OsRng};
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use rand::RngCore;
use secrecy::{ExposeSecret, Secret};

use error_stack::{Report, ResultExt};

use crate::errors;

const NONCE_LENGTH: usize = 12;

pub fn encrypt_string(
    plaintext: &str,
    encryption_key: &Secret<String>,
) -> Result<String, Report<errors::EncryptionErrorTypes>> {
    let key_bytes = encryption_key.expose_secret().as_bytes();

    let cipher = Aes256Gcm::new_from_slice(key_bytes)
        .change_context(errors::EncryptionErrorTypes::EncryptionFailed)?;

    let mut nonce_bytes = [0u8; NONCE_LENGTH];
    OsRng.fill_bytes(&mut nonce_bytes);

    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
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

    let key_bytes = encryption_key.expose_secret().as_bytes();

    let cipher = Aes256Gcm::new_from_slice(key_bytes)
        .change_context(errors::EncryptionErrorTypes::DecryptionFailed)?;

    let nonce = Nonce::from_slice(nonce_bytes);

    let decrypted = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| Report::new(errors::EncryptionErrorTypes::DecryptionFailed))
        .attach_printable("AES-GCM encryption failed")?;

    let plaintext = String::from_utf8(decrypted)
        .change_context(errors::EncryptionErrorTypes::DecryptionFailed)?;

    Ok(Secret::new(plaintext))
}
