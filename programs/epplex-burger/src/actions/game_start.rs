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
}

impl GameStart<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, params: &GameStartParams) -> Result<()> {
        params.validate_params()?;

        self.game_config.can_start_game()?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: GameStartParams) -> Result<()> {
        ctx.accounts.game_config.start(params)
    }
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameStartParams {
    pub end_timestamp: i64,
    pub vote_type: VoteType,
    pub input_type: InputType,
    pub game_prompt: String,
    pub is_encrypted: bool,
}

impl GameStartParams {
    pub fn validate_params(&self) -> Result<()> {
        let now = Clock::get().unwrap().unix_timestamp;

        if now < self.end_timestamp {
            return err!(BurgerError::InvalidGameDuration);
        }

        if self.vote_type.eq(&VoteType::None)
            || self.input_type.eq(&InputType::None)
            || self.game_prompt.is_empty()
        {
            return err!(BurgerError::InvalidStartParams);
        }

        Ok(())
    }
}
