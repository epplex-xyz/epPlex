use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenGameResetParams)]
pub struct TokenGameReset<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        seeds = [
            wen_new_standard::MEMBER_ACCOUNT_SEED,
            mint.key().as_ref()
        ],
        seeds::program = wen_new_standard::ID.key(),
        bump,
    )]
    pub group_member: Account<'info, TokenGroupMember>,

    // Only game master can handle this
    #[account(
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_GAME_CONFIG],
        bump = game_config.bump,
    )]
    pub game_config: Account<'info, GameConfig>,

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
pub struct TokenGameResetParams {}

impl TokenGameReset<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenGameResetParams) -> Result<()> {
        self.game_config.can_evaluate()?;
        self.game_config
            .check_valid_collection(&self.group_member, self.mint.key())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenGameResetParams) -> Result<()> {
        let seeds = &[SEED_PROGRAM_DELEGATE, &[ctx.accounts.update_authority.bump]];

        epplex_shared::update_token_metadata_signed(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.update_authority.to_account_info(), // the program permanent delegate
            &[&seeds[..]],
            spl_token_metadata_interface::state::Field::Key(GAME_STATE.to_string()),
            "".to_string(),
        )?;

        epplex_shared::update_token_metadata_signed(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.update_authority.to_account_info(), // the program permanent delegate
            &[&seeds[..]],
            spl_token_metadata_interface::state::Field::Key(VOTING_TIMESTAMP.to_string()),
            "".to_string(),
        )?;
        // might also need to reset immunity

        emit!(EvTokenGameReset {
            nft: ctx.accounts.mint.key(),
            game_round_id: ctx.accounts.game_config.game_round,
            reset_timestamp: Clock::get().unwrap().unix_timestamp,
        });

        Ok(())
    }
}
