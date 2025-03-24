use std::str::FromStr;

use eyre::Result;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Hash, Eq, PartialEq)]
#[serde(transparent)]
#[readonly::make]
pub struct Address {
    pubkey: Pubkey,
}

impl Address {
    pub fn from_pubkey(pubkey: Pubkey) -> Self {
        Self { pubkey }
    }

    pub const fn from_str_const(s: &'static str) -> Self {
        Self {
            pubkey: Pubkey::from_str_const(s),
        }
    }

    pub const fn from_pubkey_const(pubkey: Pubkey) -> Self {
        Self { pubkey }
    }

    pub fn pubkey(&self) -> &Pubkey {
        &self.pubkey
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pubkey)
    }
}

impl TryFrom<String> for Address {
    type Error = eyre::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let pubkey = Pubkey::from_str(&value)?;
        Ok(Self { pubkey })
    }
}

impl TryFrom<&str> for Address {
    type Error = eyre::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let pubkey = Pubkey::from_str(value)?;
        Ok(Self { pubkey })
    }
}

impl FromStr for Address {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pubkey = Pubkey::from_str(s)?;
        Ok(Self { pubkey })
    }
}

impl From<Pubkey> for Address {
    fn from(pubkey: Pubkey) -> Self {
        Self::from_pubkey(pubkey)
    }
}
