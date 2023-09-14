pub mod actions;
pub mod constants;
pub mod id;
pub mod security;
pub mod state;
pub mod types;

pub use actions::*;
pub use constants::*;
pub use id::ID;
pub use state::*;
pub use types::*;

use anchor_lang::prelude::*;

#[program]
pub mod ephemerality {
    use super::*;

    /*
     * @dev callable by operator
     */
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn token_create(ctx: Context<TokenCreate>, params: TokenCreateParams) -> Result<()> {
        TokenCreate::actuate(ctx, &params)
    }
}
