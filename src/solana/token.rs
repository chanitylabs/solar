use solana_sdk::pubkey::Pubkey;

use crate::{
    consts::tokens::{USDC_PUBKEY, USDT_PUBKEY, WSOL_ADDRESS, WSOL_DECIMALS, WSOL_PUBKEY},
    tool::{format_units, from_u64, to_u64},
};

use super::address::Address;

#[derive(thiserror::Error, Debug)]
pub enum TokenError {
    #[error("Unsupported token: {0}")]
    UnsupportedToken(Address),
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Native,
    SplToken,
    Token2022,
}

#[derive(Debug, Clone)]
pub struct Token {
    address: Address,
    decimals: u8,
    kind: TokenKind,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            TokenKind::Native => write!(f, "Native({})", self.address.to_string()),
            TokenKind::SplToken => write!(f, "SplToken({})", self.address.to_string()),
            TokenKind::Token2022 => write!(f, "Token2022({})", self.address.to_string()),
        }
    }
}

impl Token {
    pub fn with_decimals(&self, decimals: u8) -> Self {
        let cloned = self.clone();
        Self { decimals, ..cloned }
    }
    pub fn native() -> Self {
        Self {
            decimals: WSOL_DECIMALS,
            kind: TokenKind::Native,
            address: WSOL_ADDRESS,
        }
    }

    pub fn spl_token(address: &Address) -> Self {
        Self {
            decimals: 9,
            kind: TokenKind::SplToken,
            address: address.clone(),
        }
    }

    pub fn token2022(address: &Address) -> Self {
        Self {
            decimals: 9,
            kind: TokenKind::Token2022,
            address: address.clone(),
        }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }

    pub fn pubkey(&self) -> &Pubkey {
        self.address().pubkey()
    }

    pub fn ata(&self, user: &Pubkey) -> Pubkey {
        spl_associated_token_account::get_associated_token_address_with_program_id(
            user,
            self.pubkey(),
            &self.program_id(),
        )
    }

    pub fn is_spl_token(&self) -> Result<(), TokenError> {
        if matches!(self.kind, TokenKind::SplToken) {
            return Err(TokenError::UnsupportedToken(self.address().clone()));
        }

        Ok(())
    }

    pub fn program_id(&self) -> Pubkey {
        match self.kind {
            TokenKind::Native | TokenKind::SplToken => spl_token::id(),
            TokenKind::Token2022 => todo!(),
        }
    }

    pub fn is_native(&self) -> bool {
        matches!(self.kind, TokenKind::Native)
    }

    pub fn is_quote(&self) -> bool {
        let pubkey = self.pubkey();

        if self.is_native()
            || pubkey == &USDC_PUBKEY
            || pubkey == &USDT_PUBKEY
            || pubkey == &WSOL_PUBKEY
        {
            return true;
        }

        false
    }

    pub fn format(&self, amount: u64) -> f64 {
        from_u64(amount, self.decimals)
    }

    pub fn parse(&self, amount: f64) -> u64 {
        to_u64(amount, self.decimals)
    }
}
