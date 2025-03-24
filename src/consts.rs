#[cfg(feature = "solana")]
pub mod tokens {
    use crate::solana::address::Address;
    use solana_sdk::pubkey::Pubkey;

    const WSOL: &str = "So11111111111111111111111111111111111111112";
    pub const WSOL_PUBKEY: Pubkey = Pubkey::from_str_const(WSOL);
    pub const WSOL_ADDRESS: Address = Address::from_str_const(WSOL);
    pub const WSOL_DECIMALS: u8 = 9;

    const USDC: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
    pub const USDC_PUBKEY: Pubkey = Pubkey::from_str_const(USDC);
    pub const USDC_ADDRESS: Address = Address::from_str_const(USDC);
    pub const USDC_DECIMALS: u8 = 6;

    const USDT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";
    pub const USDT_PUBKEY: Pubkey = Pubkey::from_str_const(USDT);
    pub const USDT_ADDRESS: Address = Address::from_str_const(USDT);
    pub const USDT_DECIMALS: u8 = 6;
}

#[cfg(feature = "solana")]
pub mod accounts {
    pub const RAYDIUM_AMM_PROGRAM: &str = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
    pub const SOL_USDC_POOL_USDC_VAULT: &str = "HLmqeL62xR1QoZ1HKKbXRrdN1p3phKpxRMb2VVopvBBz";
    pub const SOL_USDC_POOL_SOL_VAULT: &str = "DQyrAcCrDXQ7NeoqGgDCZwBvWDcYmFCjSb9JtteuvPpz";
    pub const OPENBOOK_PROGRAM: &str = "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX";
}
