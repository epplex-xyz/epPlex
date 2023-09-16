use crate::*;
use anchor_spl::token_2022::{
    Token2022,
    spl_token_2022,
};


#[derive(Accounts)]
#[instruction(params: TokenCreateParams)]
pub struct TokenCreate<'info> {
    #[account(
        init,
        owner = token22_program.key(),
        space = Token::LEN,
        payer = payer
    )]
    /// CHECK
    pub mint: AccountInfo<'info>,
    // pub mint: Account<'info, Token>,

    #[account(
        seeds = [SEED_PROGRAM_DELEGATE],
        bump,
    )]
    /// CHECK
    pub program_delegate: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenCreateParams {}

impl TokenCreate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenCreateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: &TokenCreateParams) -> Result<()> {

        Self::add_closing_authority(ctx)?;
        // Self::add_permanent_delegate(ctx)?;

        Ok(())
    }

    fn add_permanent_delegate(ctx: Context<Self>) -> Result<()> {
        let account_infos: Vec<AccountInfo> = vec![
            ctx.accounts.mint.to_account_info(),
        ];

        let ix = spl_token_2022::instruction::initialize_permanent_delegate(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.key(),
            &ctx.accounts.program_delegate.key(),
        )?;

        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;

        Ok(())
    }

    fn add_closing_authority(ctx: Context<Self>) -> Result<()> {
        let account_infos: Vec<AccountInfo> = vec![
            ctx.accounts.mint.to_account_info(),
        ];

        let ix = spl_token_2022::instruction::initialize_mint_close_authority(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.key(),
            Some(&ctx.accounts.program_delegate.key()),
        )?;

        solana_program::program::invoke(
            &ix,
            &account_infos[..],
        )?;

        Ok(())
    }
}
