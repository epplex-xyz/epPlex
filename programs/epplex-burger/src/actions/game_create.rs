use crate::*;

#[derive(Accounts)]
pub struct GameCreate<'info> {
    #[account(
        init,
        seeds = [SEED_GAME_CONFIG],
        bump,
        payer = payer,
        space = GameConfig::LEN,
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

    pub system_program: Program<'info, System>,
}

impl GameCreate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>) -> Result<()> {
        let game_config = &mut ctx.accounts.game_config;
        **game_config = GameConfig::create(
            ctx.bumps.game_config,
            ctx.accounts.payer.key()
        );

        Ok(())
    }
}
