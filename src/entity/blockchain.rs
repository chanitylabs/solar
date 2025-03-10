use crate::encryptor::{EncryptionConfig, Encryptor};
use eyre::Result;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Hash, Copy)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum Chain {
    Solana,
}

impl TryFrom<String> for Chain {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&format!(r#""{}""#, value))
    }
}

impl std::fmt::Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self)
                .expect("failed to serialize chain")
                .trim_matches('"')
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Hash, Copy)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
#[serde(rename_all = "snake_case")]
pub enum Dex {
    RaydiumAmm,
    Pumpfun,
}

impl Dex {
    pub fn to_label(&self) -> String {
        match self {
            Dex::RaydiumAmm => "Raydium AMM".to_string(),
            Dex::Pumpfun => "Pumpfun".to_string(),
        }
    }
}

impl TryFrom<String> for Dex {
    type Error = serde_json::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        serde_json::from_str(&format!(r#""{}""#, value))
    }
}

impl std::fmt::Display for Dex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self)
                .expect("failed to serialize dex")
                .trim_matches('"')
        )
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash, Eq, PartialEq)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
#[serde(transparent)]
#[readonly::make]
pub struct Address {
    pub value: String,
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<String> for Address {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for Address {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

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

impl PrivateKey {
    pub fn encrypt(&self, config: &EncryptionConfig) -> Result<PrivateKeyEncrypted> {
        let encryptor = Encryptor::new(config);
        let encrypted = encryptor.encrypt(&self.value)?;
        Ok(PrivateKeyEncrypted { value: encrypted })
    }
}

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
    pub fn decrypt(&self, config: &EncryptionConfig) -> Result<PrivateKey> {
        let encryptor = Encryptor::new(config);
        let decrypted = encryptor.decrypt(&self.value)?;
        Ok(PrivateKey { value: decrypted })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "axum", derive(utoipa::ToSchema))]
#[serde(transparent)]
#[readonly::make]
pub struct TransactionHash {
    pub value: String,
}

impl From<String> for TransactionHash {
    fn from(value: String) -> Self {
        Self { value }
    }
}

impl From<&str> for TransactionHash {
    fn from(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl std::fmt::Display for TransactionHash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

// use std::fmt::Display;
//
// use common::encryptor::{EncryptionConfig, Encryptor};
// use eyre::Result;
//
//
//

//
// #[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
// pub struct TransactionHash(pub String);
//
// impl Display for TransactionHash {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }
//
// impl From<String> for TransactionHash {
//     fn from(value: String) -> Self {
//         Self(value)
//     }
// }
//
// impl From<&str> for TransactionHash {
//     fn from(value: &str) -> Self {
//         Self(value.to_string())
//     }
// }
//
