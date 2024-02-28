use crate::*;

#[derive(Accounts)]
#[instruction(params: GameUpdateParams)]
pub struct GameUpdate<'info> {
    #[account(
        mut,
        seeds = [SEED_GAME_CONFIG],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(
        signer,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: SystemAccount<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameUpdateParams {
    pub new_start_timestamp: i64,
}

impl GameUpdate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &GameUpdateParams) -> Result<()> {
        self.game_config.assert_game_status(GameStatus::Finished)
    }

    pub fn actuate(ctx: Context<Self>, params: GameUpdateParams) -> Result<()> {
        ctx.accounts.game_config.update(params)
    }
}
