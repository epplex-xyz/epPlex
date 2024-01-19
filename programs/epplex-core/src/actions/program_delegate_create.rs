use crate::*;

#[derive(Accounts)]
#[instruction(params: ProgramDelegateCreateParams)]
pub struct ProgramDelegateCreate<'info> {
    #[account(
        init,
        seeds = [SEED_PROGRAM_DELEGATE],
        bump,
        payer = payer,
        space = ProgramDelegate::LEN,
    )]
    pub program_delegate: Account<'info, ProgramDelegate>,

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

    // TODO: has to be gated by an admin
    pub fn actuate(ctx: Context<Self>, _params: &ProgramDelegateCreateParams) -> Result<()> {
        let program_delegate = &mut ctx.accounts.program_delegate;
        **program_delegate = ProgramDelegate::new(
            ctx.bumps.program_delegate,
        );

        Ok(())
    }
}
