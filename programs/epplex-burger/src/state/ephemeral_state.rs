use anchor_lang::solana_program::pubkey;
use anchor_lang::prelude::*;

pub const SEED_TOKEN_METADATA: &[u8] = b"metadata";

pub const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const SUPPORTED_TOKENS: [Pubkey;1] = [
    SOL
];


// Fields
pub const EXPIRY_FIELD: &str = "expirationDate";
pub const RENEWAL_FIELD: &str = "renewalAmount";
pub const FOR_SALE_FIELD: &str = "forSale";
pub const PRICE_FIELD: &str = "price";



// Prolly these need to be stored in some kind of config
// TIME
pub const ONE_WEEK: i64 = 604800;
pub const ONE_DAY: i64 = 86400;