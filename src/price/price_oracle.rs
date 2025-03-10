use std::{str::FromStr, sync::Arc, time::Duration};

use eyre::Context;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::program_pack::Pack;
use solana_sdk::pubkey::Pubkey;
use spl_token::state::Account;
use tokio::sync::RwLock;

#[derive(thiserror::Error, Debug)]
pub enum PriceOracleError {
    #[error("internal error: {0:?}")]
    InternalError(#[from] eyre::Error),
}

#[async_trait::async_trait]
pub trait PriceOracle {
    async fn get_sol_usd_price(&self) -> Result<f64, PriceOracleError>;
}

lazy_static::lazy_static! {
    pub static ref USDC_VAULT_ACCOUNT: Pubkey =
        Pubkey::from_str("HLmqeL62xR1QoZ1HKKbXRrdN1p3phKpxRMb2VVopvBBz").
            expect("correct USDC vault account");
    pub static ref USDC_DECIMALS: u8 = 6;

    pub static ref SOL_VAULT_ACCOUNT: Pubkey =
        Pubkey::from_str("DQyrAcCrDXQ7NeoqGgDCZwBvWDcYmFCjSb9JtteuvPpz").
            expect("correct SOL vault account");
    pub static ref SOL_DECIMALS: u8 = 9;
}

#[derive(Clone)]
pub struct NativePriceOracle {
    solana_rpc_url: String,
    update_interval: Duration,

    sol_usd_price: Arc<RwLock<f64>>,
}

impl NativePriceOracle {
    pub fn new(solana_rpc_url: impl Into<String>, update_interval: Duration) -> Self {
        Self {
            solana_rpc_url: solana_rpc_url.into(),
            update_interval,
            sol_usd_price: Arc::new(RwLock::new(0.0)),
        }
    }

    pub async fn run(&self) -> Result<(), PriceOracleError> {
        let update_interval = self.update_interval;
        let rpc_url = self.solana_rpc_url.clone();
        let sol_usd_price = self.sol_usd_price.clone();
        let rpc_client = RpcClient::new(rpc_url);

        {
            let mut sol_usd_price = sol_usd_price.write().await;
            let price = Self::get_sol_usd_price_native(&rpc_client)
                .await
                .context("failed to get price")?;
            *sol_usd_price = price;
        }

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(update_interval);
            loop {
                interval.tick().await;
                let mut sol_usd_price = sol_usd_price.write().await;
                let price = match Self::get_sol_usd_price_native(&rpc_client).await {
                    Ok(price) => price,
                    Err(err) => {
                        log::error!(client = "NativePriceOracle"; "failed to get price: {err:?}");
                        continue;
                    }
                };
                *sol_usd_price = price;
            }
        });

        Ok(())
    }

    async fn get_sol_usd_price_native(rpc_client: &RpcClient) -> Result<f64, PriceOracleError> {
        let sol_token_account = rpc_client
            .get_account(&SOL_VAULT_ACCOUNT)
            .await
            .context("failed to fetch USDC vault account")?;
        let sol_token_account = Account::unpack(&sol_token_account.data)
            .context("failed to unpack SOL vault account")?;

        let usdc_token_account = rpc_client
            .get_account(&USDC_VAULT_ACCOUNT)
            .await
            .context("failed to fetch SOL vault account")?;
        let usdc_token_account = Account::unpack(&usdc_token_account.data)
            .context("failed to unpack USDC vault account")?;

        let sol_balance = sol_token_account.amount as f64 / 10u64.pow(*SOL_DECIMALS as u32) as f64;
        let usdc_balance =
            usdc_token_account.amount as f64 / 10u64.pow(*USDC_DECIMALS as u32) as f64;

        let price = usdc_balance / sol_balance;

        Ok(price)
    }
}

#[async_trait::async_trait]
impl PriceOracle for NativePriceOracle {
    async fn get_sol_usd_price(&self) -> Result<f64, PriceOracleError> {
        let sol_usd_price = self.sol_usd_price.read().await;
        Ok(*sol_usd_price)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RPC_URL: &str = "https://rpc.ankr.com/solana/055fc89e9d461d5836c0dd01a50c999323c12b4d5b073690925ed92c5c677e5a";

    #[tokio::test]
    async fn test_get_sol_usd_price() {
        let oracle = NativePriceOracle::new(RPC_URL, Duration::from_secs(1));
        oracle.run().await.unwrap();
        let price = oracle.get_sol_usd_price().await.unwrap();
        assert!(price > 0.0);
    }
}
