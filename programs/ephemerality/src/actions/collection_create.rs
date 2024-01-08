use anchor_lang::prelude::borsh::BorshDeserialize;
use crate::*;

#[derive(Accounts)]
#[instruction(params: CollectionCreateParams)]
pub struct CollectionCreate<'info> {

    #[account(
        mut
    )]
    /// CHECK
    pub mint: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    #[account(
        init,
        seeds = [SEED_COLLECTION_CONFIG, params.collection_name.as_ref()],
        bump,
        payer = payer,
        space = CollectionConfig::LEN,
    )]
    /// CHECK
    pub collection_config: Account<'info, CollectionConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CollectionCreateParams {
    pub authority: Pubkey,
    pub renewal_price: u64,
    pub standard_duration: u32,
    pub grace_period: i64,
    pub treasury: Pubkey,
    pub collection_size: u32,
    pub collection_name: String,
}

impl CollectionCreate<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &CollectionCreateParams,
    ) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: CollectionCreateParams) -> Result<()> {

        Self::create_collection_mint();

        let config_acc = &mut ctx.accounts.collection_config;

        let cfg = CollectionConfig::new(
    *ctx.bumps.get("collection_config").unwrap(),
            params
        );

        config_acc.authority = cfg.authority;
        config_acc.renewal_price = cfg.renewal_price;
        config_acc.standard_duration = cfg.standard_duration;
        config_acc.grace_period = cfg.grace_period;
        config_acc.treasury = cfg.treasury;
        config_acc.collection_size = cfg.collection_size;
        config_acc.collection_name = cfg.collection_name;

        Ok(())
    }

    fn create_collection_mint() -> Result<()> {
        //TODO implement this
        Ok(())
    }
}


