use crate::*;

#[derive(Accounts)]
#[instruction(params: GameStartParams)]
pub struct GameStart<'info> {
    #[account(
        mut,
        seeds = [SEED_GAME_CONFIG],
        bump = game_config.bump,
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(
        signer,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameStartParams {
    pub end_timestamp: i64,
    pub vote_type: VoteType,
    pub input_type: InputType,
    pub game_prompt: String,
    pub is_encrypted: bool,
}

impl GameStart<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, params: &GameStartParams) -> Result<()> {
        // Check end_timestamp is in the future
        if Clock::get().unwrap().unix_timestamp < params.end_timestamp {
            return err!(BurgerError::InvalidGameDuration);
        };

        self.game_config.can_start_game()?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: GameStartParams) -> Result<()> {
        ctx.accounts.game_config.start(params)
    }
}
