use std::str::FromStr;

use eyre::{Context, Result};
use num_bigint::BigInt;
use solana_sdk::bs58;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;

use crate::entity::blockchain::{Address, PrivateKey};

pub fn from_bigint(bn: &BigInt) -> Result<u64> {
    bn.try_into().context("invalid big integer")
}

pub fn to_bigint(n: u64) -> BigInt {
    BigInt::from(n)
}
