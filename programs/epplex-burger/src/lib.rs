pub mod errors;
pub mod state;
pub mod actions;
pub mod id;

pub use errors::*;
pub use state::*;
pub use actions::*;
pub use id::ID;

use anchor_lang::prelude::*;

#[program]
pub mod epplex_burger {

    use super::*;
    
    #[access_control(ctx.accounts.validate(&ctx, &params))]
    pub fn whitelist_mint(ctx: Context<WhitelistMint>, params: WhitelistMintParams) -> Result<()> {
        WhitelistMint::actuate(ctx, params)
    }

}