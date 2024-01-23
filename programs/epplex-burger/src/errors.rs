use anchor_lang::prelude::*;

#[error_code]
pub enum BurgerError {
    #[msg("Destroy timestamp has been exceeded")]
    DestroyTimestampHasBeenExceeded,

    #[msg("Need to renew within 1 day timeframe")]
    RenewThreshold,

    #[msg("Invalid calculation")]
    InvalidCalculation,

    #[msg("Token not supported")]
    TokenNotSupported,

    #[msg("Field does not exist")]
    FieldDoesNotExist
}

