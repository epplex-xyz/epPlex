use crate::*;

#[derive(Accounts)]
#[instruction(params: GameEvaluateParams)]
pub struct GameEvaluate<'info> {
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameEvaluateParams {}

impl GameEvaluate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &GameEvaluateParams) -> Result<()> {
        self.game_config.assert_endtimestamp_passed()?;

        // Game must be INPROGRESS before we can evaluate
        self.game_config.assert_game_status(GameStatus::InProgress)
    }

    pub fn actuate(ctx: Context<Self>, _params: GameEvaluateParams) -> Result<()> {
        ctx.accounts.game_config.end(GameStatus::Evaluate)
    }
}
