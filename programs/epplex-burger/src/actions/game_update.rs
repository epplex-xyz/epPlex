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
    pub phase_start_timestamp: Option<i64>,
    pub phase_end_timestamp: Option<i64>,
    pub vote_type: Option<VoteType>,

}

impl GameUpdate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &GameUpdateParams) -> Result<()> {
        self.game_config.can_update()
    }

    pub fn actuate(ctx: Context<Self>, params: GameUpdateParams) -> Result<()> {
        ctx.accounts.game_config.update(params)
    }
}
