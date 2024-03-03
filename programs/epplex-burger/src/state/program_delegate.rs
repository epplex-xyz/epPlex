use epplex_shared::{BITS_8, DISCRIMINATOR_LENGTH};

use crate::*;

#[constant]
pub const SEED_PROGRAM_DELEGATE: &[u8] = b"BURGER_DELEGATE";

#[account]
#[derive(Default, Debug)]
pub struct ProgramDelegate {
    /// The bump, used for PDA validation.
    pub bump: u8,
}

impl ProgramDelegate {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + BITS_8;

    pub fn new(bump: u8) -> Self {
        Self { bump }
    }
}
