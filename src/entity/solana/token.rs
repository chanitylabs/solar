use super::address::Address;

pub enum Token {
    Native,
    SplToken(Address),
    Token2022(Address),
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Native => write!(f, "WSOL"),
            Token::SplToken(address) => address.fmt(f),
            Token::Token2022(address) => address.fmt(f),
        }
    }
}

impl Token {
    pub fn native() -> Self {
        Self::Native
    }

    pub fn spl_token(address: &Address) -> Self {
        Self::SplToken(address.clone())
    }

    pub fn token2022(address: &Address) -> Self {
        Self::Token2022(address.clone())
    }

    pub fn pubkey(&self) -> &Pubkey {
        match self {
            Token::Native => &WSOL_PUBKEY,
            Token::SplToken(address) => address.pubkey(),
            Token::Token2022(address) => address.pubkey(),
        }
    }

    pub fn address(&self) -> &Address {
        match self {
            Token::Native => &SOL_ADDRESS,
            Token::SplToken(address) => address,
            Token::Token2022(address) => address,
        }
    }

    pub fn ata(&self, user: &Pubkey) -> Pubkey {
        spl_associated_token_account::get_associated_token_address_with_program_id(
            user,
            self.mint(),
            &self.program_id(),
        )
    }

    pub fn is_spl_token(&self) -> Result<(), TokenError> {
        if let Token::Token2022 { address, .. } = self {
            return Err(TokenError::UnsupportedToken(address.clone()));
        }

        Ok(())
    }

    pub fn mint(&self) -> &Pubkey {
        match self {
            Token::Native => &WSOL_PUBKEY,
            Token::SplToken { pubkey, .. } => pubkey,
            Token::Token2022 { pubkey, .. } => pubkey,
        }
    }

    pub fn program_id(&self) -> Pubkey {
        match self {
            Token::Native => spl_token::id(),
            Token::SplToken { .. } => spl_token::id(),
            Token::Token2022 { .. } => todo!(),
        }
    }

    pub fn is_native(&self) -> bool {
        matches!(self, Token::Native)
    }

    pub fn is_quote(&self) -> bool {
        if self.is_native() {
            return true;
        }

        if let Token::SplToken { pubkey, .. } = self {
            if pubkey == &WSOL_PUBKEY || pubkey == &USDC_PUBKEY || pubkey == &USDT_PUBKEY {
                return true;
            }
        }

        false
    }
}
