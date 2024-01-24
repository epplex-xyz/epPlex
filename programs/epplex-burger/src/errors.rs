use anchor_lang::prelude::*;

#[error_code]
pub enum BurgerError {
    // Timestamp related
    #[msg("Expiry date has been exceeded")]
    ExpiryDateHasBeenExceeded,

    #[msg("Has not yet expired")]
    NotYetExpired,

    #[msg("Date must be in the future")]
    DateMustBeInTheFuture,

    #[msg("Need to renew within 1 day timeframe")]
    RenewThreshold,



    // Others
    #[msg("Invalid calculation")]
    InvalidCalculation,

    #[msg("Token not supported")]
    TokenNotSupported,

    #[msg("Field does not exist")]
    FieldDoesNotExist
}

