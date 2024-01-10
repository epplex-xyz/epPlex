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
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    #[account(
        init,
        seeds = [
            SEED_COLLECTION_CONFIG,
            &global_collection_config.collection_counter.to_be_bytes()
        ],
        bump,
        payer = payer,
        space = CollectionConfig::LEN,
    )]
    /// CHECK
    pub collection_config: Account<'info, CollectionConfig>,

    #[account(
        mut,
        seeds = [SEED_GLOBAL_COLLECTION_CONFIG],
        bump = global_collection_config.bump
    )]
    pub global_collection_config: Account<'info, GlobalCollectionConfig>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CollectionCreateParams {
    pub authority: Pubkey,
    pub renewal_price: u64,
    pub mint_price: u64,
    pub standard_duration: u32,
    pub grace_period: i64,
    pub treasury: Pubkey,
    pub collection_size: u32,
    pub collection_name: String,
    pub collection_symbol: String
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
        let config = &mut ctx.accounts.collection_config;
        **config = CollectionConfig::new(
            ctx.bumps.collection_config,
            params
        );

        let global_config = &mut ctx.accounts.global_collection_config;
        global_config.collection_counter += 1;

        Ok(())
    }
}


