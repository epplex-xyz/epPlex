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

    #[msg("String must not be empty")]
    EmptyString,

    #[msg("Game state must be empty")]
    GameStateMustBeEmpty,

    #[msg("Game state must not be empty")]
    GameStateMustNotBeEmpty,

    #[msg("Token not supported")]
    TokenNotSupported,

    #[msg("Field does not exist")]
    FieldDoesNotExist,

    // ----------------------------------------------- GAME ERRORS ---------------------------------------------------
    #[msg("Tring to access a finished game")]
    GameEnded,
    #[msg("Phase start greater than phase end")]
    InvalidGameDuration,
    #[msg("Phase end must be greater than current timestamp")]
    InvalidPhaseEndTS,
    #[msg("Only VoteOnce is allowed")]
    InvalidVoteMany,
}
