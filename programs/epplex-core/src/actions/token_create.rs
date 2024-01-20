use epplex_metadata::TokenMetadata;
use epplex_shared::Token2022;
use crate::*;

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
    #[account]
    /// CHECK inside CPI
    pub token_metadata: UncheckedAccount<'info, >,

    #[account(
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenCreateParams {
    pub destroy_timestamp_offset: i64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

impl TokenCreate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenCreateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenCreateParams) -> Result<()> {
       
       token_create_basic(
        ctx.accounts.mint.to_account_info().clone(),
        ctx.accounts.program_delegate.to_account_info().clone(),
        ctx.accounts.payer.to_account_info().clone(),
        ctx.accounts.rent.to_account_info().clone(),
        ctx.accounts.token22_program.to_account_info().clone(),
        &[ExtensionType::MetadataPointer]
       )?;

        // Initialize the actual mint data
        initialize_mint(
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.rent.to_account_info(),
            &ctx.accounts.token22_program.key(),
            // TODO incorrect
            &ctx.accounts.payer.key(),
            // TODO incorrect
            &ctx.accounts.payer.key(),
        )?;

        Ok(())
    }

}
