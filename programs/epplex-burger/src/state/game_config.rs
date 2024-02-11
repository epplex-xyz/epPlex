
use crate::*;

#[constant]
pub const SEED_GAME_CONFIG: &[u8] = b"GAME_CONFIG";

/// Represents each state in the lifecycle of a lotto in sequential order.
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Copy, Clone, PartialEq, Eq)]
pub enum GamePhase {
    ///
    None,
    /// Paradiso
    Announcement,
    /// Purgatorio
    Voting,
    /// Inferno
    Elimination,
}

impl Default for GamePhase {
    fn default() -> GamePhase {
        GamePhase::None
    }
}


#[account]
#[derive(Default, Debug)]
pub struct GameConfig {
    /// The bump, used for PDA validation.
    pub bump: u8,
    /// The game number
    pub game_state: u8,
    /// The game phase
    pub game_phase: GamePhase,
    /// Phase start
    pub phase_start: i64,
    /// Phase end
    pub phase_end: i64,
    /// Game master
    pub game_master: Pubkey,
}


impl GameConfig {
    pub const LEN: usize = epplex_shared::DISCRIMINATOR_LENGTH
        + epplex_shared::BITS_8
        + epplex_shared::BITS_8
        + epplex_shared::BITS_8
        + epplex_shared::BITS_64
        + epplex_shared::BITS_64
        + epplex_shared::PUBLIC_KEY_LENGTH;

    pub fn new(bump: u8, params: GameCreateParams, game_master: Pubkey) -> Self {
        Self {
            bump,
            game_state: params.game_state,
            game_phase: params.game_phase,
            phase_start: params.end_timestamp_offset,
            phase_end: params.end_timestamp_offset,
            game_master,
        }
    }


    /// Check that a ticket is claimable
    pub fn check_voting(&self) -> Result<()> {

        // if !(ctx.accounts.game_config.game_phase == GamePhase::Voting) {
        //     // TOOD return error
        // }
        Ok(())

    }

    pub fn check_phase_ended(&self) -> Result<()> {
        // if !(Clock::get().unwrap().unix_timestamp > ctx.accounts.game_config.game_phase) {
        //     return err!(LottoError::LottoTimedOut);
        // }

        Ok(())
    }
}