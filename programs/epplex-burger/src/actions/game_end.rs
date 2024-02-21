use crate::*;

#[derive(Accounts)]
pub struct GameEnd<'info> {
    #[account(
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_GAME_CONFIG],
        bump = game_config.bump,
    )]
    pub game_config: Account<'info, GameConfig>,
}

impl GameEnd<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        self.game_config.check_game_ended()?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>) -> Result<()> {
        ctx.accounts.game_config.end()
    }
}
