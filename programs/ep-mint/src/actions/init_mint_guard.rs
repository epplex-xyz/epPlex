use ephemerality::GlobalCollectionConfig;
use ephemerality::cpi::accounts::CollectionCreate;

use crate::*;

#[derive(Accounts)]
#[instruction(params: InitMintGuardParams)]
pub struct InitMintGuard<'info> {
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

    pub epplex_program: Program<'info, Ephemerality>,
    
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
    pub system_program: Program<'info, System>
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitMintGuardParams {
    collection_renewal_price: u64,
    collection_mint_price: u64,
    collection_standard_duration: u32,
    collection_grace_period: i64,
    collection_size: u32,
    collection_name: String,
    collection_symbol: String
}

impl InitMintGuard<'_> {

    pub fn validate(&self, _ctx: &Context<Self>, _params: &InitMintGuardParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: InitMintGuardParams) -> Result<()> {

        //init mint guard
        let mint_guard = &mut ctx.accounts.mint_guard;
        mint_guard.authority = ctx.accounts.creator.key();
        mint_guard.items_minted = 0;
        mint_guard.collection_counter = ctx.accounts.global_collection_config.collection_counter;
        mint_guard.bump = ctx.bumps.mint_guard;

        //create cpi
        let cpi_program = ctx.accounts.epplex_program.to_account_info();

        let cpi_accounts = CollectionCreate {
            mint: ctx.accounts.collection_mint.to_account_info(),
            program_delegate: ctx.accounts.program_delegate.to_account_info(),
            collection_config: ctx.accounts.collection_config.to_account_info(),
            global_collection_config: ctx.accounts.global_collection_config.to_account_info(),
            token22_program: ctx.accounts.token22_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            payer: ctx.accounts.creator.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        //create params
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

        ephemerality::cpi::create_collection(cpi_ctx, collection_create_params)?;
        Ok(())
    }

}