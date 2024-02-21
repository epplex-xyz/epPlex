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
        mut,
        signer,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameStartParams {
    pub game_status: GameStatus,
    pub phase_start: i64,
    pub end_timestamp_offset: i64,
    pub vote_type: VoteType,
    pub input_type: InputType,
    pub game_prompt: String,
    pub is_encrypted: bool,
}

impl GameStart<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, params: &GameStartParams) -> Result<()> {
        GameConfig::validate_create_params(params.phase_start, params.end_timestamp_offset)?;

        // Make sure game is finished
        self.game_config.can_start_game()?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: GameStartParams) -> Result<()> {
        let game_config = &mut ctx.accounts.game_config;

        let game_round: u8 = game_config.game_round + 1;

        **game_config = GameConfig::new(
            game_config.bump,
            params,
            game_round,
            ctx.accounts.payer.key(),
        );

        Ok(())
    }
}
