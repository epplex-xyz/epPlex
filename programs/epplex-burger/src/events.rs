use crate::*;

#[event]
pub struct EvTokenGameVote {
    pub answer: String,
    pub participant: Pubkey,
    pub game_round_id: u8,
    pub nft: Pubkey,
    pub vote_timestamp: i64,
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
    pub game_round_id: u8,
    pub nft: Pubkey,
    pub reset_timestamp: i64,
}

#[event]
pub struct EvTokenGameImmunity {
    pub game_round_id: u8,
    pub nft: Pubkey,
    pub participant: Pubkey,
    pub immunity_timestamp: i64,
}

#[event]
pub struct EvGameEnd {
    pub game_round_id: u8,
    pub end_timestamp: i64,
    pub game_prompt: String,
    pub game_name: String,
    pub vote_type: VoteType,
    pub input_type: InputType,
    pub public_encrypt_key: String,
    pub burn_amount: u16,
    pub submission_amount: u16,
}

#[event]
pub struct EvGameStart {
    pub game_round_id: u8,
    pub game_start_timestamp: i64,
    pub game_end_timestamp: i64,
    pub game_prompt: String,
    pub game_name: String,
    pub vote_type: VoteType,
    pub input_type: InputType,
    pub public_encrypt_key: String,
    pub burn_amount: u16,
    pub submission_amount: u16,
}
