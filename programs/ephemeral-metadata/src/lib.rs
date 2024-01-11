use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod sizes;
pub use sizes::*;

pub mod id;
pub use id::ID;

pub mod actions;
pub use actions::*;


#[program]
pub mod ephemeral_metadata {
    use super::*;

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn create_metadata(ctx: Context<CreateMetadata>,  params: CreateMetadataParams) -> Result<()> {
        CreateMetadata::actuate(ctx, params)
    }
}