use anchor_lang::prelude::borsh::BorshDeserialize;
use crate::*;

#[derive(Accounts)]
#[instruction(params: CollectionCreateParams)]
pub struct CollectionCreate<'info> {
    #[account(
        mut,
        owner = token22_program.key(),
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
        seeds = [SEED_COLLECTION_CONFIG, params.authority.key().as_ref(), params.collectionName.as_ref()]
        bump,
        payer = payer,
        space = CollectionConfig::LEN,
    )]
    /// CHECK
    pub collection_config: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CollectionCreateParams {
    pub authority: Pubkey,
    pub renewalPrice: u64,
    pub standardDuration: u32,
    pub gracePeriod: i64,
    pub treasury: Pubkey,
    pub collectionSize: u32,
    pub collectionName: Vec<u8>,
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
        CollectionConfig::new(
            *ctx.bumps.get("collection_config").unwrap(),
            params
        );

        Ok(())
    }
}
