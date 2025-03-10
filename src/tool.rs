use std::str::FromStr;

use eyre::{Context, Result};
use solana_sdk::bs58;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::entity::blockchain::{Address, PrivateKey};

pub fn parse_bn(bn: u64, decimals: u8) -> f64 {
    bn as f64 / 10u64.pow(decimals as u32) as f64
}

pub fn from_address(address: &Address) -> Result<Pubkey> {
    Pubkey::from_str(&address.value).context("invalid address")
}

pub fn from_private_key(private_key: &PrivateKey) -> Result<Keypair> {
    let bytes = bs58::decode(&private_key.value)
        .into_vec()
        .context("invalid base58 private key encoding")?;
    Keypair::from_bytes(bytes.as_ref()).context("invalid private key")
}
