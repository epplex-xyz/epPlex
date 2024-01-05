pub mod actions;
pub mod id;
pub mod security;
pub mod state;
pub mod instructions;
pub mod errors;
pub mod program_wrappers;

pub use actions::*;
pub use id::ID;
pub use state::*;
pub use instructions::*;
pub use errors::*;
pub use program_wrappers::*;

use anchor_lang::prelude::*;
use spl_token_2022::{
    extension::ExtensionType,
    state::{Mint},
};
use anchor_spl::{
    token_interface::{Mint as MintInterface}
};
use spl_token_metadata_interface::{state::TokenMetadata};

// extension::{
// transfer_fee, BaseStateWithExtensions, ExtensionType, StateWithExtensionsOwned,
// },
// use spl_token_2022::ID as SPL_TOKEN_2022_ID;

#[program]
pub mod ephemerality {
    use super::*;

    /*
     * @dev callable by operator
     */
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_create(ctx: Context<TokenCreate>, params: TokenCreateParams) -> Result<()> {
        TokenCreate::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_burn(ctx: Context<TokenBurn>, params: TokenBurnParams) -> Result<()> {
        TokenBurn::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_renew(ctx: Context<TokenRenew>, params: TokenRenewParams) -> Result<()> {
        TokenRenew::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn create_collection(ctx: Context<CollectionCreate>, params: CollectionCreateParams) -> Result<()> {
        CollectionCreate::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn program_delegate_create(ctx: Context<ProgramDelegateCreate>, params: ProgramDelegateCreateParams) -> Result<()> {
        ProgramDelegateCreate::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn program_delegate_close(ctx: Context<ProgramDelegateClose>, params: ProgramDelegateCloseParams) -> Result<()> {
        ProgramDelegateClose::actuate(ctx, &params)
    }
}
