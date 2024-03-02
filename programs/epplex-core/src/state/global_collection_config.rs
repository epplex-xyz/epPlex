use epplex_shared::{BITS_64, BITS_8, DISCRIMINATOR_LENGTH};

use crate::*;

#[constant]
pub const SEED_GLOBAL_COLLECTION_CONFIG: &[u8] = b"GLOBAL_COLLECTION";

#[account]
pub struct GlobalCollectionConfig {
    pub collection_counter: u64,
    // pub collection_counter: u128,
    pub bump: u8,
}

impl GlobalCollectionConfig {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
    // + (BITS_64 * 2)
    + BITS_64
    + BITS_8;
}
