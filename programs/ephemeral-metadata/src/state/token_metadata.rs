use crate::*;

pub const SEED_TOKEN_METADATA: &[u8] = b"metadata";

pub const EXPIRY_FIELD: &str = "expirationDate";
pub const ONE_WEEK: i64 = 604800;

#[account]
#[derive(Default, Debug)]
pub struct TokenMetadata {
    /// The authority that can sign to update the metadata
    pub update_authority: Pubkey,
    /// The associated mint, used to counter spoofing to be sure that metadata
    /// belongs to a particular mint
    pub mint: Pubkey,
    /// The longer name of the token
    pub name: String,
    /// The shortened symbol for the token
    pub symbol: String,
    /// The URI pointing to richer metadata
    pub uri: String,
    // TODO, does this match Vec<(String, String)>?
    /// Any additional metadata about the token as key-value pairs. The program
    /// must avoid storing the same key twice.
    pub additional_metadata: Vec<[String;2]>,
}

impl TokenMetadata {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + 500;
}