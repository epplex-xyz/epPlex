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
    #[msg("Endtime must be in the future")]
    IncorrectEndtime,

    #[msg("Game phase end timestamp not surpassed")]
    EndtimeNotPassed,

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

    #[msg("Game must be evaluating")]
    GameNotEvaluate,

    #[msg("Invalid game status assertion")]
    IncorrectGameStatus,

    #[msg("Game is not Finished nor None")]
    GameCannotStart,

    #[msg("Mint already submitted an answer")]
    AlreadySubmitted,

    #[msg("Invalid parameters supplied to game start instruction")]
    InvalidStartParams,

    // Input
    #[msg("Incorrect input type")]
    IncorrectInputType,

    #[msg("Input cannot be empty")]
    InputIsEmpty
}
