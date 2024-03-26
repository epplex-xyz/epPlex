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

// Alternatively  Option<GameConfig>
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameUpdateParams {
    pub phase_start_timestamp: Option<i64>,
    pub phase_end_timestamp: Option<i64>,
    pub vote_type: Option<VoteType>,
    pub token_group: Option<Pubkey>,
}

impl GameUpdate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &GameUpdateParams) -> Result<()> {
        self.game_config.can_update()
    }

    pub fn actuate(ctx: Context<Self>, params: GameUpdateParams) -> Result<()> {
        let cloned_params = params.clone();
        ctx.accounts.game_config.update(params)?;

        emit!(EvGameUpdate {
            game_round_id: ctx.accounts.game_config.game_round,
            game_start_timestamp: cloned_params.phase_start_timestamp,
            game_end_timestamp: cloned_params.phase_end_timestamp,
            vote_type: cloned_params.vote_type,
            token_group: cloned_params.token_group,
        });

        Ok(())
    }
}
