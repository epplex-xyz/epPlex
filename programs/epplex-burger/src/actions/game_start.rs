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
        mint::token_program = token22_program.key(), // ? is this check necessary
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

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

        // make sure another game isn't on going
        self.game_config.assert_game_status_none()?;

        // // ! make sure that the metadata fields are empty
        // self.game_config
        //     .check_metadata_fields_empty(&ctx.accounts.mint.to_account_info())?;

        // // ! make sure that the metadata fields are empty
        // self.game_config
        //     .check_metadata_fields_empty(&ctx.accounts.mint.to_account_info())?;

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
