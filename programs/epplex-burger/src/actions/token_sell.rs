use crate::*;
use anchor_lang::prelude::borsh::BorshDeserialize;
use epplex_shared::Token2022;

#[derive(Accounts)]
#[instruction(params: TokenSellParams)]
pub struct TokenSell<'info> {
    #[account(
        mut,
        owner = token22_program.key(),
    )]
    /// CHECK
    pub mint: AccountInfo<'info>,

    // TODO fix
    // #[account(
    //     mut,
    //     seeds = [SEED_PROGRAM_DELEGATE],
    //     bump = program_delegate.bump,
    // )]
    // pub program_delegate: Account<'info, ProgramDelegate>,
    #[account()]
    /// CHECK
    pub program_delegate: AccountInfo<'info>,

    // TODO check that this is in fact a token account for the mint
    #[account(
        mut
    )]
    /// CHECK
    pub token_account: AccountInfo<'info>,

    // TODO put constraint on payer
    #[account(mut)]
    pub payer: Signer<'info>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenSellParams {}

impl TokenSell<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &TokenSellParams,
    ) -> Result<()> {

        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: TokenSellParams) -> Result<()> {

        Ok(())
    }
}
