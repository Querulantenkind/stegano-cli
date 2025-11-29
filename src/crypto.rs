use crate::error::{Result, StegoError};
use std::io::{Read, Write};

/// Encrypt data using a passphrase (Age scrypt)
pub fn encrypt_with_passphrase(plaintext: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    let encryptor =
        age::Encryptor::with_user_passphrase(age::secrecy::SecretString::from(passphrase));

    let mut encrypted = vec![];
    let mut writer = encryptor
        .wrap_output(&mut encrypted)
        .map_err(|e| StegoError::Encryption(e.to_string()))?;

    writer
        .write_all(plaintext)
        .map_err(|e| StegoError::Encryption(e.to_string()))?;

    writer
        .finish()
        .map_err(|e| StegoError::Encryption(e.to_string()))?;

    Ok(encrypted)
}

/// Decrypt data using a passphrase
pub fn decrypt_with_passphrase(ciphertext: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    let decryptor = age::Decryptor::new(ciphertext)
        .map_err(|e| StegoError::Decryption(e.to_string()))?;

    // Create an scrypt identity from the passphrase
    let identity = age::scrypt::Identity::new(age::secrecy::SecretString::from(passphrase));

    let mut decrypted = vec![];
    let mut reader = decryptor
        .decrypt(std::iter::once(&identity as &dyn age::Identity))
        .map_err(|e| StegoError::Decryption(e.to_string()))?;

    reader
        .read_to_end(&mut decrypted)
        .map_err(|e| StegoError::Decryption(e.to_string()))?;

    Ok(decrypted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_encryption() {
        let message = b"Hello, secret world!";
        let passphrase = "test-passphrase-123";

        let encrypted = encrypt_with_passphrase(message, passphrase).unwrap();
        let decrypted = decrypt_with_passphrase(&encrypted, passphrase).unwrap();

        assert_eq!(message.as_slice(), decrypted.as_slice());
    }
}

