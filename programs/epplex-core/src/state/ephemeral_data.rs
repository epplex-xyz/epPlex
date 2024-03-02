use anchor_lang::prelude::*;

#[constant]
pub const SEED_EPHEMERAL_DATA: &[u8] = b"EPHEMERAL_DATA";


#[account]
pub struct EphemeralData {
    pub bump: u8,
    pub mint: Pubkey,
    pub rule: Pubkey,
    pub expiry_time: i64,
}

impl Space for EphemeralData {
    const INIT_SPACE: usize = epplex_shared::DISCRIMINATOR_LENGTH
        + epplex_shared::BITS_8
        + epplex_shared::PUBLIC_KEY_LENGTH
        + epplex_shared::PUBLIC_KEY_LENGTH
        + epplex_shared::BITS_64;
}
