use crate::*;

#[derive(Accounts)]
#[instruction(params: GameCreateParams)]
pub struct GameCreate<'info> {
    #[account(
        mut,
        close = payer,
        seeds = [SEED_PROGRAM_DELEGATE],
        bump = program_delegate.bump,
    )]
    pub program_delegate: Account<'info, GameConfig>,

    #[account(
        mut,
        signer,
        constraint = ADMIN_PUBKEY == payer.key()
    )]
    pub payer: SystemAccount<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameCreateParams {}

impl GameCreate<'_> {
    pub fn validate(
        &self,
        _ctx: &Context<Self>,
        _params: &GameCreateParams,
    ) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: &GameCreateParams) -> Result<()> {
        Ok(())
    }
}
