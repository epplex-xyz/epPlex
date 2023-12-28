use crate::*;

// should be derived from authority and collection name
pub const SEED_COLLECTION_CONFIG: &[u8] = b"CONFIG";

#[account]
#[derive(Default, Debug)]
pub struct CollectionConfig {
    /// The bump, used for PDA validation.
    pub bump: u8,
    pub authority: Pubkey,
    // This should be denoted in USDC
    pub renewalPrice: u64,
    pub standardDuration: u32,
    pub gracePeriod: i64,
    pub treasury: Pubkey,
    pub collectionSize: u32,
    pub collectionName: Vec<u8>,
}

impl CollectionConfig {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + BITS_8
        + PUBLIC_KEY_LENGTH
        + BITS_64
        + BITS_64
        + BITS_32
        + PUBLIC_KEY_LENGTH
        + (VEC_PREFIX + BITS_8 * COLLECTION_NAME_LENGTH);

    // TODO need to account for dynamic length in collection name
    pub fn new(bump: u8, params: CollectionCreateParams) -> Self {
        Self {
            bump,
            authority: params.authority,
            renewalPrice: params.renewalPrice,
            standardDuration: params.standardDuration,
            gracePeriod: params.gracePeriod,
            treasury: params.treasury,
            collectionSize: params.collectionSize,
            collectionName: params.collectionName,
        }
    }

}