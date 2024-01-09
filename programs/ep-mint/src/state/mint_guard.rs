use crate::*;

pub const GUARD_SEED: &[u8] = b"GUARD";

#[account]
pub struct MintGuard {
    pub authority: Pubkey,
    pub items_minted: u32,
    pub collection_counter: u64,
    pub bump: u8
}

impl MintGuard {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
    + PUBLIC_KEY_LENGTH
    + BITS_64
    + BITS_32
    + BITS_8;
}