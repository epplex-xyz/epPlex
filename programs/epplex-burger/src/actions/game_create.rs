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
        mint::token_program = token22_program.key(), // ? is this check necessary
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        mut,
        signer,
        address = ADMIN_PUBKEY
    )]
    pub payer: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct GameCreateParams {
    pub game_round: u8,
    pub game_status: GameStatus,
    pub phase_start: i64,
    pub end_timestamp_offset: i64,
    pub vote_type: VoteType,
    pub input_type: InputType,
    pub game_prompt: String,
    pub is_encrypted: bool,
}

impl GameCreate<'_> {
    pub fn validate(&self, ctx: &Context<Self>, _params: &GameCreateParams) -> Result<()> {
        // check that phases are in between each other

        self.game_config.check_phase_end_ts()?;

        self.game_config.check_duration()?;

        // // ! make sure that the metadata fields are empty
        // self.game_config
        //     .check_metadata_fields_empty(&ctx.accounts.mint.to_account_info())?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: GameCreateParams) -> Result<()> {
        let game_config = &mut ctx.accounts.game_config;
        **game_config = GameConfig::new(ctx.bumps.game_config, params, ctx.accounts.payer.key());

        Ok(())
    }
}
