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
    collection_renewal_price: u64,
    collection_mint_price: u64,
    collection_standard_duration: u32,
    collection_grace_period: i64,
    collection_size: u32,
    collection_name: String,
    collection_symbol: String
}

impl WhitelistMint<'_> {

    pub fn validate(&self, _ctx: &Context<Self>, _params: &WhitelistMintParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: WhitelistMintParams) -> Result<()> {
        Ok(())
    }

}
