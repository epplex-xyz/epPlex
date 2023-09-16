use crate::*;

pub const SEED_PROGRAM_DELEGATE: &[u8] = b"PROGRAM_DELEGATE";

#[derive(Accounts)]
#[instruction(params: ProgramDelegateCreateParams)]
pub struct ProgramDelegateCreate<'info> {
    #[account(
        init,
        seeds = [SEED_PROGRAM_DELEGATE],
        bump,
        payer = payer,
        space = 8,
    )]
    /// CHECK
    pub program_delegate: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ProgramDelegateCreateParams {}

impl ProgramDelegateCreate<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &ProgramDelegateCreateParams,
    ) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: &ProgramDelegateCreateParams) -> Result<()> {
        Ok(())
    }
}
