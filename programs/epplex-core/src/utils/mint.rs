use crate::*;

pub const COLLECTION_ID_FIELD: &str = "collection_id";
pub const MINT_COUNT_FIELD: &str = "mint_count";

pub const SEED_MINT: &[u8] = b"MINT";

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenCreateParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub additional_metadata: Vec<[String;2]>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenCollectionCreateParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub collection_id: u64,
    pub additional_metadata: Vec<[String;2]>,
}