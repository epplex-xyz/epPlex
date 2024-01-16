use anchor_lang::prelude::*;

#[error_code]
pub enum MintError {
    #[msg("Collection already minted out")]
    CollectionMintedOut
}

#[error_code]
pub enum WithdrawError {
    #[msg("The signer is not the authority")]
    InvalidAuthority
}