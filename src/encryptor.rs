use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use eyre::{Context, Error, Result, eyre};
use rand::RngCore;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EncryptionConfig {
    pub secret: String,
}

impl From<String> for EncryptionConfig {
    fn from(value: String) -> Self {
        Self { secret: value }
    }
}

impl From<&str> for EncryptionConfig {
    fn from(value: &str) -> Self {
        Self {
            secret: value.to_string(),
        }
    }
}

pub struct Encryptor {
    cipher: Aes256Gcm,
}

impl Encryptor {
    pub fn new(config: &EncryptionConfig) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(config.secret.as_bytes());

        let hash_result = hasher.finalize();
        let key = Key::<Aes256Gcm>::from_slice(&hash_result);

        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String> {
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| eyre!("encryption failed: {e}"))?;

        let mut combined = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
        combined.extend_from_slice(&nonce_bytes);
        combined.extend_from_slice(&ciphertext);

        Ok(BASE64.encode(combined))
    }

    pub fn decrypt(&self, encrypted: &str) -> Result<String> {
        let encrypted_data = BASE64.decode(encrypted).context("Base64 decoding failed")?;

        if encrypted_data.len() < 12 {
            return Err(eyre!("Encrypted data too short"));
        }

        let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| eyre!("decryption failed: {e}"))?;

        String::from_utf8(plaintext).map_err(Error::from)
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let secret = "test-password";
        let encryptor = Encryptor::new(&secret.into());
        let original = "Hello, World!";

        let encrypted = encryptor.encrypt(original).unwrap();
        let decrypted = encryptor.decrypt(&encrypted).unwrap();

        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_different_passwords() {
        let encryptor1 = Encryptor::new(&"password1".into());
        let encryptor2 = Encryptor::new(&"password2".into());
        let message = "Secret message";

        let encrypted = encryptor1.encrypt(message).unwrap();
        assert!(encryptor2.decrypt(&encrypted).is_err());
    }

    #[test]
    fn test_invalid_data() {
        let encryptor = Encryptor::new(&"password".into());
        assert!(encryptor.decrypt("invalid-base64!").is_err());
        assert!(encryptor.decrypt("aGVsbG8=").is_err()); // too short
    }
}
