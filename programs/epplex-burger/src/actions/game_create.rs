use crate::*;

#[derive(Accounts)]
#[instruction(params: GameCreateParams)]
pub struct GameCreate<'info> {
    #[account(
        init,
        seeds = [SEED_GAME_CONFIG],
        bump,
        payer = payer,
        space = GameConfig::LEN,
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account(
        mut,
        signer,
        address = ADMIN_PUBKEY
    )]
    pub payer: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameCreateParams {
    pub game_state: u8,
    pub game_phase: GamePhase,
    pub phase_start: i64,
    pub end_timestamp_offset: i64,
    pub vote_type: VoteType,
    pub input_type: InputType,
    pub game_prompt: String
}

impl GameCreate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &GameCreateParams) -> Result<()> {
        // check that phases are in between each other

        self.game_config.check_phase_end_ts()?;

        self.game_config.check_duration()?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: GameCreateParams) -> Result<()> {
        let game_config = &mut ctx.accounts.game_config;
        **game_config = GameConfig::new(
            ctx.bumps.game_config,
            params,
            ctx.accounts.payer.key(),
        );

        Ok(())
    }
}
