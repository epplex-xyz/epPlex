use epplex_shared::{BITS_8, DISCRIMINATOR_LENGTH};
use crate::*;

#[constant]
pub const SEED_BURGER_METADATA: &[u8] = b"GAME_CONFIG";

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


#[account]
#[derive(Default, Debug)]
pub struct GameConfig {
    /// The bump, used for PDA validation.
    pub bump: u8,
    /// The game number
    pub game_state: u8,
    pub game_phase: GamePhase,
    pub phase_end: i64,
    pub phase_start: i64,
    pub game_master: Pubkey,
}


impl GameConfig {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + BITS_8;

    pub fn new(bump: u8) -> Self {
        Self {bump}
    }
}