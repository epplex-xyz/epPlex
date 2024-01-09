pub mod state;
pub use state::*;

pub mod actions;
pub use actions::*;

pub mod id;
pub use id::ID;

pub mod program_wrappers;
pub use program_wrappers::*;

use anchor_lang::prelude::*;
use ephemerality::{
    cpi::accounts::{CollectionCreate, TokenCreate},
    EphemeralMetadata, program::Ephemerality,
    SEED_TOKEN_METADATA,
    SEED_COLLECTION_CONFIG

};
use ephemerality::{CollectionCreateParams, CollectionConfig, TokenCreateParams};
use spl_token_2022;
use anchor_spl::{
    associated_token::AssociatedToken,
};

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
}