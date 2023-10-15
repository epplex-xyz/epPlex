pub mod actions;
pub mod id;
pub mod security;
pub mod state;
pub mod instructions;
pub mod errors;

pub use actions::*;
pub use constants::*;
pub use id::ID;
pub use state::*;
pub use instructions::*;
pub use errors::*;

use anchor_lang::prelude::*;
use spl_token_2022;
use spl_token_metadata_interface;

#[derive(Clone)]
pub struct Token2022;

impl Id for Token2022 {
    fn id() -> Pubkey {
        spl_token_2022::ID
    }
}


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
    pub fn program_delegate_create(ctx: Context<ProgramDelegateCreate>, params: ProgramDelegateCreateParams) -> Result<()> {
        ProgramDelegateCreate::actuate(ctx, &params)
    }

    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn program_delegate_close(ctx: Context<ProgramDelegateClose>, params: ProgramDelegateCloseParams) -> Result<()> {
        ProgramDelegateClose::actuate(ctx, &params)
    }
}
