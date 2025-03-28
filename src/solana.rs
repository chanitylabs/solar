use eyre::{Context, Result};
use num_bigint::BigInt;

pub fn from_bigint(bn: &BigInt) -> Result<u64> {
    bn.try_into().context("invalid big integer")
}

pub fn to_bigint(n: u64) -> BigInt {
    BigInt::from(n)
}
