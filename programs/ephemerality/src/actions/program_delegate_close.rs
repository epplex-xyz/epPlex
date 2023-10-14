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

    #[account(mut)]
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
        // TODO:
        // Currently anyone would be able to invoke this instruction
        // need to restrict this instruction to some kind of admin
        Ok(())
    }
}
