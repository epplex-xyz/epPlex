use crate::*;

#[constant]
pub const SEED_GAME_CONFIG: &[u8] = b"GAME_CONFIG";

pub const GAME_QUESTION_LENGTH: usize = 150;

/// Represents game activity.
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum GameStatus {
    #[default]
    None,
    InProgress, // active
    Finished,   // inactive
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum VoteType {
    #[default]
    VoteOnce,
    VoteMany,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
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
    pub phase_start: i64,
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
    /// Amount of burgers who perished
    pub burn_amount: u16,
    /// Amount of burgers who submitted an answer
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
        + epplex_shared::BITS_16
        + epplex_shared::BITS_16;

    pub fn new(bump: u8, params: GameStartParams, game_round: u8, game_master: Pubkey) -> Self {
        Self {
            bump,
            game_round,
            game_status: params.game_status,
            phase_start: params.phase_start,
            phase_end: params.end_timestamp_offset,
            vote_type: params.vote_type,
            input_type: params.input_type,
            game_prompt: params.game_prompt,
            game_master,
            is_encrypted: params.is_encrypted,
            burn_amount: 0,
            submission_amount: 0,
        }
    }

    /// Check that a ticket is claimable
    pub fn check_voting(&self) -> Result<()> {
        // ? should we check this
        if self.game_status == GameStatus::Finished {
            return err!(BurgerError::GameFinished);
        }

        //

        Ok(())
    }

    /// make sure that `phase_end > phase_start`
    pub fn assert_valid_duration(&self) -> Result<()> {
        if self.phase_end < self.phase_start {
            return err!(BurgerError::InvalidGameDuration);
        }

        Ok(())
    }

    /// make sure that `phase_end > current timestamp`
    pub fn check_phase_end_ts(&self) -> Result<()> {
        let now = Clock::get().unwrap().unix_timestamp;

        if self.phase_end < now {
            return err!(BurgerError::InvalidGameDuration);
        }

        Ok(())
    }

    // /// disallows transition in the last phase `ELIMINATION` of the game
    // pub fn check_game_ended(&self) -> Result<()> {
    //     if self.game_status.eq(&GameStatus::Finished) {
    //         return err!(BurgerError::GameEnded);
    //     }

    //     Ok(())
    // }

    // make sure that the game config was reset before starting another game
    pub fn assert_game_status_none(&self) -> Result<()> {
        if self.game_status != GameStatus::None {
            return err!(BurgerError::GameInProgress);
        }

        Ok(())
    }

    // make sure that a game is not in progress when calling create IX
    pub fn assert_game_finished(&self) -> Result<()> {
        if self.game_status != GameStatus::Finished {
            return err!(BurgerError::GameInProgress);
        }

        Ok(())
    }

    pub fn assert_game_in_progress(&self) -> Result<()> {
        if self.game_status != GameStatus::InProgress {
            return err!(BurgerError::GameFinished);
        }

        Ok(())
    }

    pub fn assert_metadata_fields_empty(&self, mint: &AccountInfo) -> Result<()> {
        let game_state = fetch_metadata_field(GAME_STATE, mint)?;
        let vote_ts = fetch_metadata_field(VOTING_TIMESTAMP, mint)?;

        if !game_state.is_empty() && game_state != GAME_STATE_PLACEHOLDER {
            return err!(BurgerError::ExpectedEmptyField);
        }

        if !vote_ts.is_empty() && vote_ts != VOTING_TIMESTAMP_PLACEHOLDER {
            msg!("vote timestamp field {:?}", vote_ts);
            return err!(BurgerError::ExpectedEmptyField);
        }

        Ok(())
    }

    /// check that the metadata fields are not empty or filled with initial default values
    pub fn assert_metadata_fields_filled(&self, mint: &AccountInfo) -> Result<()> {
        let game_state = fetch_metadata_field(GAME_STATE, mint)?;

        if game_state.is_empty() || game_state == GAME_STATE_PLACEHOLDER {
            msg!("game status {:?}", game_state);
            // default game state means user hasn't participated in the game
            return err!(BurgerError::InvalidGameState);
        }

        let expiry_ts = fetch_metadata_field(EXPIRY_FIELD, mint)?;
        if expiry_ts.is_empty() {
            return err!(BurgerError::InvalidExpiryTS);
        }

        let voting_ts = fetch_metadata_field(VOTING_TIMESTAMP, mint)?;
        if voting_ts.is_empty() || voting_ts == VOTING_TIMESTAMP_PLACEHOLDER {
            return err!(BurgerError::InvalidExpiryTS);
        }

        Ok(())
    }

    pub fn check_mint_expiry_ts(&self, mint: &AccountInfo) -> Result<()> {
        let expiry_ts = fetch_metadata_field(EXPIRY_FIELD, mint)?;
        let now = Clock::get().unwrap().unix_timestamp;

        if expiry_ts.is_empty() {
            return err!(BurgerError::InvalidExpiryTS);
        }

        if now > expiry_ts.parse::<i64>().unwrap_or_default() {
            return err!(BurgerError::InvalidExpiryTS);
        }

        Ok(())
    }

    pub fn validate_create_params(phase_start: i64, phase_end: i64) -> Result<()> {
        let now = Clock::get().unwrap().unix_timestamp;

        // check the phase end
        if phase_end < now {
            return err!(BurgerError::InvalidGameDuration);
        }

        // check duration
        if phase_end < phase_start {
            return err!(BurgerError::InvalidGameDuration);
        }

        Ok(())
    }

    // first time initialization

    /// Bump burn amount
    pub fn bump_burn_amount(&mut self) -> Result<()> {
        self.burn_amount = self
            .burn_amount
            .checked_add(1)
            .ok_or(BurgerError::InvalidCalculation)?;

        Ok(())
    }

    /// check_encrypted
    pub fn check_encrypted(&self, message: &String) -> Result<()> {
        if self.is_encrypted {
            if message.len() != ENCRYPTED_LENTH {
                return err!(BurgerError::RequiresEncryption);
            }
        }

        Ok(())
    }
}
