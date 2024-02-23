pub mod errors;
pub use errors::*;

pub mod state;
pub use state::*;

pub mod actions;
pub use actions::*;

pub mod id;
pub use id::ID;

pub mod utils;
pub use utils::*;

use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount},
    // ID as TOKEN_2022_PROGRAM_ID alternatively
    token_2022::{self},
    token_interface::{Mint as MintInterface, TokenAccount as TokenAccountInterface},
};
use epplex_shared;
use epplex_shared::Token2022;
use spl_token_metadata_interface::state::TokenMetadata;

#[program]
pub mod epplex_burger {
    use super::*;

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn whitelist_mint(ctx: Context<WhitelistMint>, params: WhitelistMintParams) -> Result<()> {
        WhitelistMint::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn collection_mint(
        ctx: Context<CollectionMint>,
        params: CollectionMintParams,
    ) -> Result<()> {
        CollectionMint::actuate(ctx, params)
    }

    /*
     * Token actions
     */
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

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_burn(ctx: Context<TokenBurn>, params: TokenBurnParams) -> Result<()> {
        TokenBurn::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_game_vote(ctx: Context<TokenGameVote>, params: TokenGameVoteParams) -> Result<()> {
        TokenGameVote::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_game_reset(
        ctx: Context<TokenGameReset>,
        params: TokenGameResetParams,
    ) -> Result<()> {
        TokenGameReset::actuate(ctx, params)
    }

    /*
     * Game create
     */
    #[access_control(ctx.accounts.validate(&ctx,))]
    pub fn game_create(ctx: Context<GameCreate>) -> Result<()> {
        GameCreate::actuate(ctx)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn game_start(ctx: Context<GameStart>, params: GameStartParams) -> Result<()> {
        GameStart::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn game_end(ctx: Context<GameEnd>) -> Result<()> {
        GameEnd::actuate(ctx)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn game_close(ctx: Context<GameClose>) -> Result<()> {
        GameClose::actuate(ctx)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn game_update(ctx: Context<GameUpdate>, params: GameUpdateParams) -> Result<()> {
        GameUpdate::actuate(ctx, params)
    }

    /*
     * Program Delegate
     */
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn program_delegate_create(
        ctx: Context<ProgramDelegateCreate>,
        params: ProgramDelegateCreateParams,
    ) -> Result<()> {
        ProgramDelegateCreate::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn program_delegate_close(
        ctx: Context<ProgramDelegateClose>,
        params: ProgramDelegateCloseParams,
    ) -> Result<()> {
        ProgramDelegateClose::actuate(ctx, &params)
    }
}
