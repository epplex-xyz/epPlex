use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenGameVoteParams)]
pub struct TokenGameVote<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        token::mint = mint,
        token::authority = payer,
        token::token_program = token22_program.key(),
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccountInterface>>, // Used to verify owner

    #[account(
        seeds = [
            SEED_BURGER_METADATA,
            mint.key().as_ref()
        ],
        bump = token_metadata.bump
    )]
    pub token_metadata: Account<'info, BurgerMetadata>,

    #[account(
        seeds = [SEED_GAME_CONFIG],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account()]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = update_authority.bump
    )]
    pub update_authority: Account<'info, ProgramDelegate>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenGameVoteParams {
    pub message: String,
}

impl TokenGameVote<'_> {
    pub fn validate(&self, ctx: &Context<Self>, params: &TokenGameVoteParams) -> Result<()> {
        ctx.accounts.game_config.check_encrypted(params.message)?;
        // ! check that the game is in progress
        // self.game_config.check_game_in_progress()?;

        // // ! check that the metadata fields are empty
        // // ? what if the user can cast multiple votes
        // self.game_config
        //     .check_metadata_fields_empty(&ctx.accounts.mint.to_account_info())?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: TokenGameVoteParams) -> Result<()> {
        let seeds = &[SEED_PROGRAM_DELEGATE, &[ctx.accounts.update_authority.bump]];
        // Update game state
        epplex_shared::update_token_metadata_signed(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.update_authority.to_account_info(), // the program permanent delegate
            &[&seeds[..]],
            spl_token_metadata_interface::state::Field::Key(GAME_STATE.to_string()),
            params.message,
        )?;

        // Record voting timestamp
        let now = Clock::get().unwrap().unix_timestamp;
        epplex_shared::update_token_metadata_signed(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.update_authority.to_account_info(), // the program permanent delegate
            &[&seeds[..]],
            spl_token_metadata_interface::state::Field::Key(VOTING_TIMESTAMP.to_string()),
            now.to_string(),
        )?;

        // TODO we should only increase the submission amount if the NFT metadata is not in a default state

        Ok(())
    }
}
