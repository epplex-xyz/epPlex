use crate::*;

/// The primary lotto account
#[account]
#[derive(Default, Debug)]
pub struct ProgramDelegate {
    /// The bump, used for PDA validation.
    pub bump: u8,
}

impl ProgramDelegate {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + BITS_8;

    pub fn new(bump: u8) -> Self {
        Self {bump}
    }

}
