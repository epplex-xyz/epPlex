use anchor_lang::prelude::*;

#[error_code]
pub enum BurgerError {
    #[msg("Destroy timestamp has been exceeded")]
    DestroyTimestampHasBeenExceeded,

    #[msg("Invalid calculation")]
    InvalidCalculation,

    #[msg("Token not supported")]
    TokenNotSupported
}

