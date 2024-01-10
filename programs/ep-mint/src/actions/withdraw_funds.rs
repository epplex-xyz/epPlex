use crate::*;

#[derive(Accounts)]
pub struct WithdrawFunds<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            GUARD_SEED,
            collection_config.key().as_ref()
        ],
        bump = mint_guard.bump
    )]
    pub mint_guard: Account<'info, MintGuard>,

    #[account(
        seeds = [
            SEED_COLLECTION_CONFIG,
            &mint_guard.collection_counter.to_be_bytes(),
        ],
        seeds::program = ephemerality::ID.key(),
        bump = collection_config.bump
    )]
    pub collection_config: Account<'info, CollectionConfig>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WithdrawFundsParams {
    amount: u64
}

impl WithdrawFunds<'_> {

    pub fn validate(&self, ctx: &Context<Self>) -> Result<()> {

        if ctx.accounts.mint_guard.authority != ctx.accounts.authority.key() {
            return err!(WithdrawError::InvalidAuthority);
        }

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: WithdrawFundsParams) -> Result<()> {

        let mint_guard = ctx.accounts.mint_guard.to_account_info();
        let authority = ctx.accounts.authority.to_account_info();

        mint_guard.sub_lamports(params.amount)?;
        authority.add_lamports(params.amount)?;

        Ok(())
    }

}