pub mod errors;
pub mod state;
pub mod actions;
pub mod id;

pub use errors::*;
pub use state::*;
pub use actions::*;
pub use id::ID;

pub mod program_wrappers;
pub use program_wrappers::*;

use anchor_lang::prelude::*;
use ephemerality::{program::Ephemerality, SEED_COLLECTION_CONFIG};
use ephemerality::{CollectionCreateParams, CollectionConfig};
use anchor_spl::associated_token::AssociatedToken;

#[program]
pub mod ep_mint {

    use super::*;
    
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn init_mint_guard(ctx: Context<InitMintGuard>, params: InitMintGuardParams) -> Result<()> {
        InitMintGuard::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn mint_from_collection(ctx: Context<MintFromCollection>) -> Result<()> {
        MintFromCollection::actuate(ctx)
    }

    #[access_control(ctx.accounts.validate(&ctx))]
    pub fn withdraw_funds(ctx: Context<WithdrawFunds>, params: WithdrawFundsParams) -> Result<()> {
        WithdrawFunds::actuate(ctx, params)
    }
}