use crate::*;

#[derive(Accounts)]
#[instruction(params: CollectionCloseParams)]
pub struct CollectionClose<'info> {
    #[account(
        mut,
        close = payer,
        seeds = [
            SEED_COLLECTION_CONFIG,
            params.collection_id.to_le_bytes().as_ref()
        ],
        bump = collection_config.bump,
    )]
    pub collection_config: Account<'info, CollectionConfig>,

    #[account(
        mut,
        constraint = epplex_shared::ADMINS.contains(
            &payer.key()
        )
    )]
    pub payer: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CollectionCloseParams {
    collection_id: u64,
}

impl CollectionClose<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &CollectionCloseParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: CollectionCloseParams) -> Result<()> {
        Ok(())
    }
}
