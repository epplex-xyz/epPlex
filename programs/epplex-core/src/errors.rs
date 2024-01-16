use crate::*;

#[error_code]
pub enum EphemeralityError {
    #[msg("Invalid calculation")]
    InvalidCalculation,
    #[msg("Destroy timestamp has not been exceeded")]
    DestroyTimestampNotExceeded
}

#[error_code]
pub enum MintError {
    #[msg("unauthorized mint authority")]
    UnauthorizedMintAuthority,

    #[msg("the given treasury account does not match with the configured treasury")]
    InvalidTreasuryAccount
}