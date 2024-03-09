use crate::*;

#[event]
pub struct EvTokenGameVote {
    pub answer: String,
    pub participant: Pubkey,
}

#[event]
pub struct EvTokenGameBurn {
    pub answer: String,
    pub participant: Pubkey,
}

#[event]
pub struct EvTokenGameReset {
    pub answer: String,
    pub participant: Pubkey,
}
