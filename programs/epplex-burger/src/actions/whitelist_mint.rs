use epplex_core::GlobalCollectionConfig;
use epplex_core::cpi::accounts::CollectionCreate;
use epplex_shared::Token2022;

use crate::*;

#[derive(Accounts)]
#[instruction(params: WhitelistMintParams)]
pub struct WhitelistMint<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WhitelistMintParams {

}

impl WhitelistMint<'_> {

    pub fn validate(&self, _ctx: &Context<Self>, _params: &WhitelistMintParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: WhitelistMintParams) -> Result<()> {
        Ok(())
    }

}
