use crate::*;
use anchor_lang::solana_program::pubkey;

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
        constraint = [pubkey!("epADzKVW5kb3hjUhKuxdmyASNKYt4Cb1ccLGvr5cuzh")].contains(
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