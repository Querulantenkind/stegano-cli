use crate::error::{Result, StegoError};
use age::secrecy::ExposeSecret;
use std::io::{BufRead, Read, Write};

/// Generated keypair with public and private keys
pub struct Keypair {
    pub identity: String,  // Private key (AGE-SECRET-KEY-1...)
    pub recipient: String, // Public key (age1...)
}

/// Generate a new Age x25519 keypair
pub fn generate_keypair() -> Keypair {
    let identity = age::x25519::Identity::generate();
    let recipient = identity.to_public();

    Keypair {
        identity: identity.to_string().expose_secret().to_string(),
        recipient: recipient.to_string(),
    }
}

/// Parse a recipient public key string
fn parse_recipient(s: &str) -> Result<age::x25519::Recipient> {
    s.parse::<age::x25519::Recipient>()
        .map_err(|e| StegoError::Encryption(format!("Invalid recipient '{}': {}", s, e)))
}

/// Parse recipients from a file (one per line)
pub fn parse_recipients_file(path: &str) -> Result<Vec<age::x25519::Recipient>> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut recipients = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        recipients.push(parse_recipient(line)?);
    }

    if recipients.is_empty() {
        return Err(StegoError::Encryption(
            "No valid recipients found in file".into(),
        ));
    }

    Ok(recipients)
}

/// Parse an identity (private key) from a file
pub fn parse_identity_file(path: &str) -> Result<age::x25519::Identity> {
    let content = std::fs::read_to_string(path)?;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("AGE-SECRET-KEY-") {
            return line
                .parse::<age::x25519::Identity>()
                .map_err(|e| StegoError::Decryption(format!("Invalid identity: {}", e)));
        }
    }

    Err(StegoError::Decryption(
        "No valid identity found in file".into(),
    ))
}

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

/// Encrypt data for multiple recipients using public keys
pub fn encrypt_with_recipients(
    plaintext: &[u8],
    recipient_keys: &[String],
    recipient_file: Option<&str>,
) -> Result<Vec<u8>> {
    let mut recipients: Vec<age::x25519::Recipient> = Vec::new();

    // Parse recipient strings
    for key in recipient_keys {
        recipients.push(parse_recipient(key)?);
    }

    // Parse recipients from file
    if let Some(path) = recipient_file {
        recipients.extend(parse_recipients_file(path)?);
    }

    if recipients.is_empty() {
        return Err(StegoError::Encryption("No recipients specified".into()));
    }

    // Convert to iterator of trait objects
    let recipient_refs: Vec<&dyn age::Recipient> = recipients
        .iter()
        .map(|r| r as &dyn age::Recipient)
        .collect();

    let encryptor = age::Encryptor::with_recipients(recipient_refs.into_iter())
        .map_err(|e| StegoError::Encryption(e.to_string()))?;

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

/// Decrypt data using an identity file (private key)
pub fn decrypt_with_identity(ciphertext: &[u8], identity_path: &str) -> Result<Vec<u8>> {
    let identity = parse_identity_file(identity_path)?;

    let decryptor = age::Decryptor::new(ciphertext)
        .map_err(|e| StegoError::Decryption(e.to_string()))?;

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
    fn roundtrip_passphrase_encryption() {
        let message = b"Hello, secret world!";
        let passphrase = "test-passphrase-123";

        let encrypted = encrypt_with_passphrase(message, passphrase).unwrap();
        let decrypted = decrypt_with_passphrase(&encrypted, passphrase).unwrap();

        assert_eq!(message.as_slice(), decrypted.as_slice());
    }

    #[test]
    fn keypair_generation() {
        let keypair = generate_keypair();

        assert!(keypair.identity.starts_with("AGE-SECRET-KEY-"));
        assert!(keypair.recipient.starts_with("age1"));
    }

    #[test]
    fn roundtrip_pubkey_encryption() {
        let message = b"Secret message for recipient";
        let keypair = generate_keypair();

        // Encrypt with public key
        let encrypted =
            encrypt_with_recipients(message, &[keypair.recipient.clone()], None).unwrap();

        // Write identity to temp file for decryption
        let temp_dir = std::env::temp_dir();
        let identity_path = temp_dir.join("test_identity.txt");
        std::fs::write(&identity_path, &keypair.identity).unwrap();

        // Decrypt with private key
        let decrypted =
            decrypt_with_identity(&encrypted, identity_path.to_str().unwrap()).unwrap();

        assert_eq!(message.as_slice(), decrypted.as_slice());

        // Cleanup
        std::fs::remove_file(identity_path).ok();
    }
}
