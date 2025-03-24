pub mod address;
pub mod chain;
pub mod dex;
pub mod private_key;
pub mod token;

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
