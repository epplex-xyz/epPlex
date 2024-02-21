use epplex_shared::{BITS_8, DISCRIMINATOR_LENGTH};
use crate::*;

#[constant]
pub const SEED_BURGER_METADATA: &[u8] = b"burgermetadata";


/// Reserve 200 bytes
pub const GAME_STATE_PLACEHOLDER: &str =
    "99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999";

/// Length of 20 characters unixtimestamp
pub const VOTING_TIMESTAMP_PLACEHOLDER: &str = "99999999999999999999";


pub const ENCRYPTED_LENTH: usize = 172;


#[account]
#[derive(Default, Debug)]
pub struct BurgerMetadata {
    /// The bump, used for PDA validation.
    pub bump: u8,
}

impl BurgerMetadata {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + BITS_8;

    pub fn new(bump: u8) -> Self {
        Self {bump}
    }
}