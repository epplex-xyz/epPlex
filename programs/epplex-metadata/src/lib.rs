use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod id;
pub use id::ID;

pub mod actions;
pub use actions::*;

pub mod metadata_update_utils;
pub use metadata_update_utils::*;

use spl_token_metadata_interface::state::{TokenMetadata as TM, Field};

#[program]
pub mod epplex_metadata {
    use super::*;
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn metadata_create(ctx: Context<MetadataCreate>, params: MetadataCreateParams) -> Result<()> {
        MetadataCreate::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn metadata_create_separate(ctx: Context<MetadataCreateSeparate>, params: MetadataCreateSeparateParams) -> Result<()> {
        MetadataCreateSeparate::actuate(ctx, params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn metadata_update(ctx: Context<MetadataUpdate>, params: UpdateFieldData) -> Result<()> {
        MetadataUpdate::actuate(ctx, params)
    }
}
