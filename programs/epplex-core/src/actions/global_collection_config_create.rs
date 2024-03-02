use crate::*;

#[derive(Accounts)]
pub struct GlobalCollectionConfigCreate<'info> {
    #[account(
        init,
        seeds = [
            SEED_GLOBAL_COLLECTION_CONFIG
        ],
        bump,
        payer = payer,
        space = GlobalCollectionConfig::LEN
    )]
    pub global_collection_config: Account<'info, GlobalCollectionConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,

}

impl GlobalCollectionConfigCreate<'_> {
    pub fn validate(&self,_ctx: &Context<Self>,) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>) -> Result<()> {
        let config = &mut ctx.accounts.global_collection_config;
        config.collection_counter = 0;
        config.bump = ctx.bumps.global_collection_config;

        Ok(())
    }
}