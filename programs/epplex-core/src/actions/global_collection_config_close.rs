use crate::*;

#[derive(Accounts)]
pub struct GlobalCollectionConfigClose<'info> {
    #[account(
        mut,
        close = payer,
        seeds =  [SEED_GLOBAL_COLLECTION_CONFIG],
        bump = global_collection_config.bump,
    )]
    pub global_collection_config: Account<'info, GlobalCollectionConfig>,

    #[account(
        mut,
        constraint = epplex_shared::ADMINS.contains(
            &payer.key()
        )
    )]
    pub payer: Signer<'info>,
}

impl GlobalCollectionConfigClose<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>) -> Result<()> {
        Ok(())
    }
}
