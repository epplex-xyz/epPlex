use crate::*;

/// The primary lotto account
#[account]
#[derive(Default, Debug)]
pub struct Token {
}


impl Token {
    pub const LEN: usize = DISCRIMINATOR_LENGTH;

    pub fn new(_bump: u8) -> Self {
        Self {}
    }

}
