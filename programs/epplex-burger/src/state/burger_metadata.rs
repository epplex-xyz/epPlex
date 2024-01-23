use epplex_shared::{BITS_8, DISCRIMINATOR_LENGTH};
use crate::*;

pub const SEED_BURGER_METADATA: &[u8] = b"burgermetadata";

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