use crate::*;

#[derive(Accounts)]
#[instruction(params: GameEndParams)]
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

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameEndParams {}

impl GameEnd<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &GameEndParams) -> Result<()> {
        self.game_config.assert_endtimestamp_passed()?;

        // GameState must be EVALUATE before we can end game
        self.game_config.assert_game_status(GameStatus::Evaluate)
    }

    pub fn actuate(ctx: Context<Self>, _params: GameEndParams) -> Result<()> {
        ctx.accounts.game_config.end(GameStatus::Finished)
    }
}
