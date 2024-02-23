use crate::*;

#[derive(Accounts)]
pub struct GameClose<'info> {
    #[account(
        mut,
        close = payer,
        seeds = [SEED_GAME_CONFIG],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(
        mut,
        signer,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: SystemAccount<'info>,
}

impl GameClose<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>) -> Result<()> {
        Ok(())
    }
}
