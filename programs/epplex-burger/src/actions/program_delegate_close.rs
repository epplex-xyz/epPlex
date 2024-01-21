use crate::*;

#[derive(Accounts)]
#[instruction(params: ProgramDelegateCloseParams)]
pub struct ProgramDelegateClose<'info> {
    #[account(
        mut,
        close = payer,
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

    #[account(
        mut,
        signer,
        constraint = BURN_AUTH == payer
    )]
    pub payer: SystemAccount<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProgramDelegateCloseParams {}

impl ProgramDelegateClose<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &ProgramDelegateCloseParams,
    ) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: &ProgramDelegateCloseParams) -> Result<()> {
        Ok(())
    }
}
