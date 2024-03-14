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
        ctx.accounts.game_config.end(GameStatus::Finished)?;

        emit!(EvGameEnd {
            game_round_id: ctx.accounts.game_config.game_round,
            end_timestamp: Clock::get().unwrap().unix_timestamp,
            game_prompt: ctx.accounts.game_config.game_prompt.clone(),
            game_name: ctx.accounts.game_config.game_name.clone(),
            vote_type: ctx.accounts.game_config.vote_type,
            input_type: ctx.accounts.game_config.input_type,
            public_encrypt_key: ctx.accounts.game_config.public_encrypt_key.clone(),
            burn_amount: ctx.accounts.game_config.burn_amount,
            submission_amount: ctx.accounts.game_config.submission_amount
        });

        Ok(())
    }
}
