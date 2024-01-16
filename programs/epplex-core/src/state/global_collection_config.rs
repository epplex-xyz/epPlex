use epplex_shared::{DISCRIMINATOR_LENGTH, BITS_64, BITS_8};

use crate::*;

pub const SEED_GLOBAL_COLLECTION_CONFIG: &[u8] = b"GLOBAL_COLLECTION";

#[account]
pub struct GlobalCollectionConfig {
    pub collection_counter: u64,
    pub bump: u8
}

impl GlobalCollectionConfig {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
    + BITS_64
    + BITS_8;
}