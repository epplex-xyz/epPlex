use crate::*;
use anchor_lang::prelude::borsh::BorshDeserialize;

#[derive(Accounts)]
#[instruction(params: TokenFreezeParams)]
pub struct TokenFreeze<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        mut,
        token::mint = mint.key(),
        token::token_program = token22_program.key(),
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccountInterface>>,

    #[account(
        seeds = [
            SEED_BURGER_METADATA,
            mint.key().as_ref()
        ],
        bump = token_metadata.bump
    )]
    pub token_metadata: Account<'info, BurgerMetadata>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = permanent_delegate.bump
    )]
    pub permanent_delegate: Account<'info, ProgramDelegate>,

    #[account(
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenFreezeParams {}

impl TokenFreeze<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenFreezeParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenFreezeParams) -> Result<()> {
        let seeds = &[
            SEED_PROGRAM_DELEGATE,
            &[ctx.accounts.permanent_delegate.bump],
        ];
        anchor_spl::token_interface::freeze_account(CpiContext::new_with_signer(
            ctx.accounts.token22_program.to_account_info(),
            anchor_spl::token_interface::FreezeAccount {
                mint: ctx.accounts.mint.to_account_info().clone(),
                account: ctx.accounts.token_account.to_account_info().clone(),
                authority: ctx.accounts.permanent_delegate.to_account_info().clone(),
            },
            &[&seeds[..]],
        ))?;

        Ok(())
    }
}
