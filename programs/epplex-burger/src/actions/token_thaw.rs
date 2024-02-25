use crate::*;
use anchor_lang::prelude::borsh::BorshDeserialize;
use epplex_shared::Token2022;

#[derive(Accounts)]
#[instruction(params: TokenThawParams)]
pub struct TokenThaw<'info> {
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

    // Gate who can execute this
    #[account(
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenThawParams {}

impl TokenThaw<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &TokenThawParams,
    ) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenThawParams) -> Result<()> {
        Ok(())
    }
}
