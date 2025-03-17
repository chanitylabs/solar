use num_bigint::BigInt;
use solana_sdk::bs58;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use std::str::FromStr;

use eyre::{Context, Result};

use crate::entity::blockchain::{Address, PrivateKey};

pub fn from_address(address: &Address) -> Result<Pubkey> {
    Pubkey::from_str(&address.value).context("invalid address")
}

pub fn from_private_key(private_key: &PrivateKey) -> Result<Keypair> {
    let bytes = bs58::decode(&private_key.value)
        .into_vec()
        .context("invalid base58 private key encoding")?;
    Keypair::from_bytes(bytes.as_ref()).context("invalid private key")
}

pub fn from_bigint(bn: &BigInt) -> Result<u64> {
    bn.try_into().context("invalid big integer")
}

pub fn to_bigint(n: u64) -> BigInt {
    BigInt::from(n)
}
