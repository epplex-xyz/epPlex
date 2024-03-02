use crate::*;

#[error_code]
pub enum EphemeralityError {
    #[msg("Invalid calculation")]
    InvalidCalculation,
    #[msg("Destroy timestamp has not been exceeded")]
    DestroyTimestampNotExceeded,

    #[msg("You don't have the authority to perform this action")]
    EscalatedAuthority,
    #[msg("Overflow")]
    Overflow,
    #[msg("The membership has not yet expired! Note that the grace period is 14 hours.")]
    NotExpired,
    #[msg("The membership has already expired! You cannot remove time from it.")]
    AlreadyExpired,
}

#[error_code]
pub enum MintError {
    #[msg("unauthorized mint authority")]
    UnauthorizedMintAuthority,

    #[msg("the given treasury account does not match with the configured treasury")]
    InvalidTreasuryAccount
}


