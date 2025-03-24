#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
pub struct PrivateKey {
    pub value: String,
}

impl From<String> for PrivateKey {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for PrivateKey {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl std::fmt::Display for PrivateKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(feature = "encryptor")]
mod private_key_encrypted {
    use super::*;

    use crate::encryptor::{EncryptionConfig, Encryptor};

    #[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
    pub struct PrivateKeyEncrypted {
        pub value: String,
    }

    impl From<String> for PrivateKeyEncrypted {
        fn from(value: String) -> Self {
            Self { value }
        }
    }

    impl PrivateKeyEncrypted {
        pub fn decrypt(&self, config: &EncryptionConfig) -> eyre::Result<PrivateKey> {
            let encryptor = Encryptor::new(config);
            let decrypted = encryptor.decrypt(&self.value)?;
            Ok(PrivateKey { value: decrypted })
        }
    }

    impl PrivateKey {
        #[cfg(feature = "encryptor")]
        pub fn encrypt(&self, config: &EncryptionConfig) -> eyre::Result<PrivateKeyEncrypted> {
            let encryptor = Encryptor::new(config);
            let encrypted = encryptor.encrypt(&self.value)?;
            Ok(PrivateKeyEncrypted { value: encrypted })
        }
    }
}
