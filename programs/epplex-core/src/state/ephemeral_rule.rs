use anchor_lang::prelude::*;


#[constant]
pub const SEED_EPHEMERAL_AUTH: &[u8] = b"EPHEMERAL_AUTH";

#[constant]
pub const SEED_EPHEMERAL_RULE: &[u8] = b"EPHEMERAL_RULE";

#[account]
pub struct EphemeralRule {
    pub bump: u8,
    pub seed: u64,
    pub rule_creator: Pubkey,
    pub renewal_price: u64,
    pub treasury: Pubkey,
}

impl Space for EphemeralRule {
    const INIT_SPACE: usize = epplex_shared::DISCRIMINATOR_LENGTH
        + epplex_shared::BITS_8
        + epplex_shared::BITS_64
        + epplex_shared::PUBLIC_KEY_LENGTH
        + epplex_shared::BITS_64
        + epplex_shared::PUBLIC_KEY_LENGTH;
}