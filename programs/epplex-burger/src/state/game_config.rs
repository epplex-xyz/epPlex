use crate::*;

#[constant]
pub const SEED_GAME_CONFIG: &[u8] = b"GAME_CONFIG";

pub const GAME_QUESTION_LENGTH: usize = 150;

/// Represents each state in the lifecycle of a lotto in sequential order.
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum GamePhase {
    #[default]
    None,
    /// Paradiso
    Announcement,
    /// Purgatorio
    Voting,
    /// Inferno
    Elimination,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum VoteType {
    #[default]
    VoteOnce,
    VoteMany
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum InputType {
    #[default]
    Choice,
    Text,
    Number
}


#[account]
#[derive(Default, Debug)]
pub struct GameConfig {
    /// The bump, used for PDA validation.
    pub bump: u8,
    /// The game number
    pub game_round: u8,
    /// The game phase
    pub game_phase: GamePhase,
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
        + epplex_shared::BITS_16;

    pub fn new(bump: u8, params: GameCreateParams, game_master: Pubkey) -> Self {
        Self {
            bump,
            game_round: params.game_round,
            game_phase: params.game_phase,
            phase_start: params.end_timestamp_offset,
            phase_end: params.end_timestamp_offset,
            vote_type: params.vote_type,
            input_type: params.input_type,
            game_prompt: params.game_prompt,
            game_master,
            is_encrypted: params.is_encrypted,
            burn_amount: 0,
        }
    }

    /// Check that a ticket is claimable
    pub fn check_voting(&self) -> Result<()> {
        if !(self.game_phase == GamePhase::None) && self.vote_type == VoteType::VoteOnce {
            return err!(BurgerError::InvalidVoteMany);

        }

        Ok(())
    }

    /// make sure that `phase_end > phase_start`
    pub fn check_duration(&self) -> Result<()> {
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

    /// disallows transition in the last phase `ELIMINATION` of the game
    pub fn check_game_ended(&self) -> Result<()> {
        if self.game_phase.eq(&GamePhase::Elimination) {
            return err!(BurgerError::GamePhaseLastStage);
        }

        Ok(())
    }

    /// Bump burn amount
    pub fn bump_burn_amount(&mut self) -> Result<()> {
        self.burn_amount = self.burn_amount
            .checked_add(1)
            .ok_or(BurgerError::InvalidCalculation)?;

        Ok(())
    }

}
