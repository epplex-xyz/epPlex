use crate::*;

#[constant]
pub const SEED_GAME_CONFIG: &[u8] = b"GAME_CONFIG";

pub const GAME_QUESTION_LENGTH: usize = 150;
pub const PUBLIC_ENCRYPT_KEY_LENGTH: usize = 250;

/// Represents game activity.
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum GameStatus {
    #[default]
    None,
    InProgress, // active
    Evaluate,   // evaluating game results
    Finished,   // inactive
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum VoteType {
    #[default]
    None,
    VoteOnce,
    VoteMany,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    None,
    Choice,
    Text,
    Number,
}

#[account]
#[derive(Default, Debug)]
pub struct GameConfig {
    /// The bump, used for PDA validation.
    pub bump: u8,
    /// The game number
    pub game_round: u8,
    /// The game status
    pub game_status: GameStatus,
    /// Phase start
    pub phase_start_timestamp: i64,
    /// Phase end
    pub phase_end: i64,
    /// Game master
    pub game_master: Pubkey,
    /// Game vote type
    pub vote_type: VoteType,
    /// Game input type
    pub input_type: InputType,
    /// Game question of 150 characters
    pub game_prompt: String,
    /// Is answer encrypted
    pub is_encrypted: bool,
    /// Public encrypt key
    pub public_encrypt_key: String,
    /// Total amount of burgers who perished
    pub burn_amount: u16,
    /// Amount of burgers who submitted an answer within a round
    pub submission_amount: u16,
}

impl GameConfig {
    pub const LEN: usize = epplex_shared::DISCRIMINATOR_LENGTH
        + epplex_shared::BITS_8
        + epplex_shared::BITS_8
        + epplex_shared::BITS_8
        + epplex_shared::BITS_64
        + epplex_shared::BITS_64
        + epplex_shared::PUBLIC_KEY_LENGTH
        + epplex_shared::BITS_8
        + epplex_shared::BITS_8
        + (epplex_shared::VEC_PREFIX + GAME_QUESTION_LENGTH * epplex_shared::UTF_SIZE)
        + epplex_shared::BITS_8
        + (epplex_shared::VEC_PREFIX + PUBLIC_ENCRYPT_KEY_LENGTH * epplex_shared::UTF_SIZE)
        + epplex_shared::BITS_16
        + epplex_shared::BITS_16;

    pub fn create(bump: u8, game_master: Pubkey) -> Self {
        Self {
            bump,
            game_round: 0,
            game_status: GameStatus::None,
            phase_start_timestamp: 0,
            phase_end: 0,
            vote_type: VoteType::None,
            input_type: InputType::Choice,
            game_prompt: "".to_string(),
            game_master,
            is_encrypted: false,
            public_encrypt_key: "".to_string(),
            burn_amount: 0,
            submission_amount: 0,
        }
    }

    pub fn start(&mut self, params: GameStartParams) -> Result<()> {
        self.game_round = self
            .game_round
            .checked_add(1)
            .ok_or(BurgerError::InvalidCalculation)?;
        self.game_status = GameStatus::InProgress;
        self.phase_start_timestamp = Clock::get().unwrap().unix_timestamp;
        self.phase_end = params.end_timestamp;
        self.vote_type = params.vote_type;
        self.input_type = params.input_type;
        self.game_prompt = params.game_prompt;
        self.is_encrypted = params.is_encrypted;
        self.public_encrypt_key = params.public_encrypt_key;

        Ok(())
    }

    pub fn end(&mut self, game_status: GameStatus) -> Result<()> {
        self.game_status = game_status;
        self.phase_start_timestamp = 0;
        self.phase_end = 0;
        self.vote_type = VoteType::None;
        self.input_type = InputType::None;
        self.game_prompt = "".to_string();
        self.is_encrypted = false;
        self.public_encrypt_key = "".to_string();
        self.submission_amount = 0;

        Ok(())
    }

    pub fn update(&mut self, params: GameUpdateParams) -> Result<()> {
        self.phase_start_timestamp = params.new_start_timestamp;

        Ok(())
    }

    /// Check for game end
    pub fn check_game_ended(&self, game_status: GameStatus) -> Result<()> {
        if self.phase_end < Clock::get().unwrap().unix_timestamp {
            return err!(BurgerError::InvalidGameDuration);
        }

        // Game must be in progress before we can end game
        self.assert_game_status(game_status)?;

        Ok(())
    }

    /// Can only start game if NOT in progress
    pub fn can_start_game(&self) -> Result<()> {
        if self.game_status.eq(&GameStatus::InProgress) {
            return err!(BurgerError::GameInProgress)
        }

        Ok(())
    }

    // Fail if current game status does not match the specified state
    pub fn assert_game_status(&self, status: GameStatus) -> Result<()> {
        match status {
            // If is finished then continue
            GameStatus::Finished => {
                if self.game_status.ne(&GameStatus::Finished) {
                    return err!(BurgerError::GameNotFinished);
                }
            },
            // If in progress then continue
            GameStatus::InProgress => {
                if self.game_status.ne(&GameStatus::InProgress) {
                    return err!(BurgerError::GameNotInProgress);
                }
            },
            // If evaluating then continue
            GameStatus::Evaluate => {
                if self.game_status.ne(&GameStatus::Evaluate) {
                    return err!(BurgerError::GameNotEvaluate);
                }
            }
            _ => {
                return err!(BurgerError::IncorrectGameStatus);
            }
        }

        Ok(())
    }

    /// Bump burn amount
    pub fn bump_burn_amount(&mut self) -> Result<()> {
        self.burn_amount = self
            .burn_amount
            .checked_add(1)
            .ok_or(BurgerError::InvalidCalculation)?;

        Ok(())
    }

        /// Bump burn amount
    pub fn bump_submission_amount(&mut self, game_state: String) -> Result<()> {
        if game_state.is_empty() {
            self.submission_amount = self
                .submission_amount
                .checked_add(1)
                .ok_or(BurgerError::InvalidCalculation)?;
        }

        Ok(())
    }

    /// Check if vote type is encryption
    pub fn check_encrypted(&self, message: &String) -> Result<()> {
        if self.is_encrypted {
            if message.len() != ENCRYPTED_LENTH {
                return err!(BurgerError::RequiresEncryption);
            }
        }

        Ok(())
    }

    /// Check if vote type is encryption
    pub fn validate_input(&self, message: &String) -> Result<()> {
        self.check_encrypted(message)?;

        if message.is_empty() {
            return err!(BurgerError::InputIsEmpty);
        }

        match self.input_type {
            InputType::Choice => {
                // No checks for now


                // let choice = message.parse::<u8>().unwrap();
                // // Max choice is 10
                // if choice > 10 {
                //     return err!(BurgerError::IncorrectInputType)
                // }
            },
            InputType::Number => {
                // Panic if fails to convert
                message.parse::<u64>().unwrap();

            },
            InputType::Text => {
                // No checks for now
            },
            InputType::None => return err!(BurgerError::RequiresEncryption)
        };


        Ok(())
    }

    /// Check for vote eligibility
    pub fn check_vote_eligibility(&self, game_state: String ) -> Result<()> {
        if self.vote_type.eq(&VoteType::VoteOnce) && !game_state.is_empty() {
            return err!(BurgerError::AlreadySubmitted);
        }

        Ok(())
    }
}
