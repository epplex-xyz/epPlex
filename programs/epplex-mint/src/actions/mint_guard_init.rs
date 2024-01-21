use epplex_core::GlobalCollectionConfig;
use epplex_core::cpi::accounts::CollectionCreate;
use epplex_shared::Token2022;

use crate::*;

#[derive(Accounts)]
#[instruction(params: MintGuardInitParams)]
pub struct MintGuardInit<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        seeds = [
            GUARD_SEED,
            collection_config.key().as_ref()
        ],
        payer = creator,
        space = 8 + MintGuard::LEN,
        bump
    )]
    pub mint_guard: Account<'info, MintGuard>,
    
    #[account(mut)]
    /// CHECK
    pub collection_mint: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK
    pub collection_config: AccountInfo<'info>,

    #[account(mut)]
    /// CHECK
    pub global_collection_config: Account<'info, GlobalCollectionConfig>,

    #[account(mut)]
    /// CHECK
    pub program_delegate: AccountInfo<'info>,
    pub token22_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
    pub epplex_program: Program<'info, EpplexCore>,
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MintGuardInitParams {
    collection_renewal_price: u64,
    collection_mint_price: u64,
    collection_standard_duration: u32,
    collection_grace_period: i64,
    collection_size: u32,
    collection_name: String,
    collection_symbol: String
}


impl MintGuardInit<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &MintGuardInitParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: MintGuardInitParams) -> Result<()> {
        // Init mint guard
        // TODO put into state account
        let mint_guard = &mut ctx.accounts.mint_guard;
        mint_guard.authority = ctx.accounts.creator.key();
        mint_guard.items_minted = 0;
        mint_guard.collection_counter = ctx.accounts.global_collection_config.collection_counter;
        mint_guard.bump = ctx.bumps.mint_guard;

        // Create cpi
        let cpi_program = ctx.accounts.epplex_program.to_account_info();
        let cpi_accounts = CollectionCreate {
            collection_config: ctx.accounts.collection_config.to_account_info(),
            global_collection_config: ctx.accounts.global_collection_config.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            payer: ctx.accounts.creator.to_account_info()
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Create params
        let collection_create_params = CollectionCreateParams {
            authority: mint_guard.key(),
            renewal_price: params.collection_renewal_price,
            mint_price: params.collection_mint_price,
            standard_duration: params.collection_standard_duration,
            grace_period: params.collection_grace_period,
            treasury: mint_guard.key(),
            collection_size: params.collection_size,
            collection_name: params.collection_name,
            collection_symbol: params.collection_symbol
        };

        epplex_core::cpi::collection_create(cpi_ctx, collection_create_params)?;

        Ok(())
    }
}
