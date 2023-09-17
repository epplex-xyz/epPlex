use crate::*;

#[derive(Accounts)]
#[instruction(params: ProgramDelegateCloseParams)]
pub struct ProgramDelegateClose<'info> {
    #[account(
        mut,
        close = payer,
        seeds = [SEED_PROGRAM_DELEGATE],
        bump,
    )]
    /// CHECK
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
        _params: &ProgramDelegateCreateParams,
    ) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: &ProgramDelegateCloseParams) -> Result<()> {
        Ok(())
    }
}
