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
    pub end_timestamp: i64,
    pub vote_type: VoteType,
    pub input_type: InputType,
    pub game_prompt: String,
    pub is_encrypted: bool,
}

impl GameUpdate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &GameUpdateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: GameUpdateParams) -> Result<()> {
        Ok(())
    }
}
