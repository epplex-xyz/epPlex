pub mod errors;
pub use errors::*;

pub mod state;
pub use state::*;

pub mod actions;
pub use actions::*;

pub mod id;
pub use id::ID;

pub mod token_utils;
pub use token_utils::*;
use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, TokenAccount},
    token_2022::{self},
    token_interface::{
        Mint as MintInterface,
        TokenAccount as TokenAccountInterface
    }
};
use spl_token_metadata_interface::state::TokenMetadata;
use epplex_shared::Token2022;

#[program]
pub mod epplex_burger {
    use epplex_core::{ProgramDelegateClose, ProgramDelegateCloseParams, ProgramDelegateCreate, ProgramDelegateCreateParams};
    use super::*;
    
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn whitelist_mint(ctx: Context<WhitelistMint>, params: WhitelistMintParams) -> Result<()> {
        WhitelistMint::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_buy(ctx: Context<TokenBuy>, params: TokenBuyParams) -> Result<()> {
        TokenBuy::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_renew(ctx: Context<TokenRenew>, params: TokenRenewParams) -> Result<()> {
        TokenRenew::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_delist(ctx: Context<TokenDelist>, params: TokenDelistParams) -> Result<()> {
        TokenDelist::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_sell(ctx: Context<TokenSell>, params: TokenSellParams) -> Result<()> {
        TokenSell::actuate(ctx, params)
    }

    // // Todo this should simply be updating the metadata field
    // #[access_control(ctx.accounts.validate(&ctx, &params))]
    // pub fn vote_cast(ctx: Context<VoteCast>, params: VoteCastParams) -> Result<()> {
    //     VoteCast::actuate(ctx, params)
    // }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn program_delegate_create(ctx: Context<ProgramDelegateCreate>, params: ProgramDelegateCreateParams) -> Result<()> {
        ProgramDelegateCreate::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn program_delegate_close(ctx: Context<ProgramDelegateClose>, params: ProgramDelegateCloseParams) -> Result<()> {
        ProgramDelegateClose::actuate(ctx, &params)
    }
}