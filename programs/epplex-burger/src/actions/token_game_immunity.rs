use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenGameImmunityParams)]
pub struct TokenGameImmunity<'info> {
    // Actual burger nft
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
            wen_new_standard::MEMBER_ACCOUNT_SEED,
            mint.key().as_ref()
        ],
        seeds::program = wen_new_standard::ID.key(),
        bump,
    )]
    pub group_member: Account<'info, wen_new_standard::TokenGroupMember>,

    #[account(
        seeds = [
            SEED_GAME_CONFIG
        ],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,

    // TODO maybe change to game_master
    #[account(
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = permanent_delegate.bump
    )]
    pub permanent_delegate: Account<'info, ProgramDelegate>,

    pub token22_program: Program<'info, Token2022>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenGameImmunityParams {}

impl TokenGameImmunity<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenGameImmunityParams) -> Result<()> {
        self.game_config.assert_game_status(GameStatus::Evaluate)?;
        self.game_config
            .check_valid_collection(&self.group_member, self.mint.key())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenGameImmunityParams) -> Result<()> {
        let seeds = &[
            SEED_PROGRAM_DELEGATE,
            &[ctx.accounts.permanent_delegate.bump],
        ];

        epplex_shared::update_token_metadata_signed(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.permanent_delegate.to_account_info(), // the program permanent delegate
            &[&seeds[..]],
            spl_token_metadata_interface::state::Field::Key(IMMUNITY.to_string()),
            "true".to_string(),
        )?;

        emit!(EvTokenGameImmunity {
            game_round_id: ctx.accounts.game_config.game_round,
            nft: ctx.accounts.mint.key(),
            participant: epplex_shared::get_token_account_owner(
                &ctx.accounts.token_account.to_account_info()
            )?,
            immunity_timestamp: Clock::get().unwrap().unix_timestamp,
        });

        Ok(())
    }
}
