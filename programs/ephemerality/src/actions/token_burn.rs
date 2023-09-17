use crate::*;

// #[derive(Clone)]
// pub struct Token2022;
//
// impl anchor_lang::Id for Token2022 {
//     fn id() -> Pubkey {
//         spl_token_2022::ID
//     }
// }

#[derive(Accounts)]
#[instruction(params: TokenBurnParams)]
pub struct TokenBurn<'info> {
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

    // TODO check that this is in fact a token account for th emint
    #[account(
        mut
    )]
    /// CHECK
    pub token_account: AccountInfo<'info>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenBurnParams {}

impl TokenBurn<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &TokenBurnParams,
    ) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: &TokenBurnParams) -> Result<()> {
        burn_token(
            &ctx.accounts.mint,
            &ctx.accounts.token_account,
            ctx.accounts.token22_program.key(),
            &ctx.accounts.program_delegate,
        )?;

        close_mint(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint,
            &ctx.accounts.program_delegate.to_account_info(),
            &ctx.accounts.program_delegate,
        )?;

        Ok(())
    }
}
