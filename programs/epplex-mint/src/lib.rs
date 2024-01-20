pub mod errors;
pub mod state;
pub mod actions;
pub mod id;

pub use errors::*;
pub use state::*;
pub use actions::*;
pub use id::ID;

use anchor_lang::prelude::*;
use epplex_core::{program::EpplexCore, SEED_COLLECTION_CONFIG};
use epplex_core::{CollectionCreateParams, CollectionConfig};
use anchor_spl::associated_token::AssociatedToken;

#[program]
pub mod epplex_mint {
    use super::*;
    
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn mint_guard_init(ctx: Context<MintGuardInit>, params: MintGuardInitParams) -> Result<()> {
        MintGuardInit::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn collection_mint_from(ctx: Context<CollectionMintFrom>, params: CollectionMintFromParams) -> Result<()> {
        CollectionMintFrom::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn funds_withdraw(ctx: Context<FundsWithdraw>, params: FundsWithdrawParams) -> Result<()> {
        FundsWithdraw::actuate(ctx, params)
    }
}