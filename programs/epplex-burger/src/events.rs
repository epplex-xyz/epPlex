use crate::*;

#[event]
pub struct EvTokenGameVote {
    pub answer: String,
    pub participant: Pubkey,
    pub game_round_id: u8,
    pub nft: Pubkey,
}

#[event]
pub struct EvTokenGameBurn {
    pub nft: Pubkey,
    pub game_round_id: u8,
    pub participant: Pubkey,
    pub burn_timestamp: i64,
}

#[event]
pub struct EvTokenGameReset {
    pub answer: String,
    pub participant: Pubkey,
}
