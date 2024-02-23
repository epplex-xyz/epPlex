use anchor_lang::solana_program::pubkey;
use anchor_lang::prelude::*;

pub const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
pub const USDC: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
pub const DEVNET_USDC: Pubkey = pubkey!("FGwbc2XjEC27hLx2QkgUSNTCAC9W8YhD9SK3wcgtzJCb");
pub const BONK: Pubkey = pubkey!("DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263");

pub const SUPPORTED_TOKENS: [Pubkey; 4] = [
    SOL,
    USDC,
    BONK,
    DEVNET_USDC,
];


// Fields
pub const EXPIRY_FIELD: &str = "expirationDate"; // should just add onto this
pub const GAME_STATE: &str = "gameState";
pub const VOTING_TIMESTAMP: &str = "votingTimestamp";



// Prolly these need to be stored in some kind of config
// TIME
pub const ONE_WEEK: i64 = 604800;
pub const ONE_DAY: i64 = 86400;