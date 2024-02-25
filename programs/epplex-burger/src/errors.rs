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

    #[msg("Non-operator attempts to use program")]
    NonOperator,

    // ----------------------------------------------- GAME ERRORS ---------------------------------------------------
    #[msg("Phase start greater than phase end")]
    InvalidGameDuration,
    #[msg("Phase end must be greater than current timestamp")]
    InvalidPhaseEndTS,
    #[msg("Only VoteOnce is allowed")]
    InvalidVoteMany,
    #[msg("Empty expiry timestamp on metadata account")]
    InvalidExpiryTS,
    #[msg("Empty game state field on metadata account")]
    InvalidGameState,
    #[msg("Expected additional metadata field to be empty")]
    ExpectedEmptyField,
    #[msg("Message was not encrypted")]
    RequiresEncryption,
    #[msg("Game must be finished")]
    GameNotFinished,
    #[msg("Game must be in progress")]
    GameNotInProgress,
    #[msg("Invalid game status assertion")]
    IncorrectGameStatus,
    #[msg("Game is in progress")]
    GameInProgress,
    #[msg("Mint already submitted an answer")]
    AlreadySubmitted,
    #[msg("Invalid parameters supplied to game start instruction")]
    InvalidStartParams,
    #[msg("Incorrect input type")]
    IncorrectInputType,
    #[msg("Input cannot be empty")]
    InputIsEmpty
}
