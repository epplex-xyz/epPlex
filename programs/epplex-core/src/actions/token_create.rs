use epplex_shared::Token2022;
use crate::*;
use crate::mint::TokenCreateParams;

#[derive(Accounts)]
#[instruction(params: TokenCreateParams)]
pub struct TokenCreate<'info> {
    // TODO: is unchecked account correct?
    #[account(mut, signer)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    // #[account(
    //     seeds = [
    //         SEED_TOKEN_METADATA,
    //         mint.key().as_ref()
    //     ],
    //     seeds::program = epplex_metadata::ID.key(),
    //     bump
    // )]
    #[account()]
    /// CHECK inside CPI
    pub token_metadata: UncheckedAccount<'info, >,

    // TODO: is unchecked account correct?
    #[account()]
    /// CHECK
    pub permanent_delegate: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>
}



impl TokenCreate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenCreateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenCreateParams) -> Result<()> {
       // token_create_basic(
       //  ctx.accounts.mint.to_account_info().clone(),
       //  ctx.accounts.permanent_delegate.to_account_info().clone(),
       //  ctx.accounts.payer.to_account_info().clone(),
       //  ctx.accounts.rent.to_account_info().clone(),
       //  ctx.accounts.token22_program.to_account_info().clone(),
       //  &[ExtensionType::MetadataPointer]
       // )?;

        // Initialize the actual mint data
        initialize_mint(
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.rent.to_account_info(),
            &ctx.accounts.token22_program.key(),
            // TODO incorrect mint auth
            &ctx.accounts.payer.key(),
            // TODO incorrect freeze auth
            &ctx.accounts.payer.key(),
        )?;

        Ok(())
    }

}
