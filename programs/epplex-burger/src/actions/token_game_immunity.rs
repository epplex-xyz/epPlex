use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenGameImmunityParams)]
pub struct TokenGameImmunity<'info> {
    // Mint to be burned in exchange for immmunity
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint_immunity.decimals == 0,
        constraint = mint_immunity.supply == 1,
    )]
    pub mint_immunity: Box<InterfaceAccount<'info, MintInterface>>,

    // ATA to be burned in exchange for immmunity
    #[account(
        token::mint = mint_immunity,
        token::authority = payer,
        token::token_program = token22_program.key(),
    )]
    pub token_account_immunity: Box<InterfaceAccount<'info, TokenAccountInterface>>, // Used to verify owner

    #[account(
        seeds = [
            SEED_BURGER_METADATA,
            mint_immunity.key().as_ref()
        ],
        bump = token_metadata_immunity.bump
    )]
    pub token_metadata_immunity: Account<'info, BurgerMetadata>,

    // Actual burger nft
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    // ATA to be burned in exchange for immmunity
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
        seeds = [
            SEED_GAME_CONFIG
        ],
        bump = game_config.bump
    )]
    pub game_config: Account<'info, GameConfig>,

    #[account()]
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
        // TODO, Immunity can only be enabled once game has finished
        self.game_config.assert_game_status(GameStatus::Finished)
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenGameImmunityParams) -> Result<()> {
        // Immunity token is also a T22
        burn_token(
            &ctx.accounts.mint_immunity.to_account_info(),
            &ctx.accounts.token_account_immunity.to_account_info(),
            ctx.accounts.token22_program.key(),
            &ctx.accounts.permanent_delegate.to_account_info(),
        )?;

        // Close immuntiy token mint
        close_mint(
            ctx.accounts.token22_program.key(),
            &ctx.accounts.mint_immunity.to_account_info(),
            // Currently rent collector is hardcoded to be the Program Delegaate
            &ctx.accounts.payer.to_account_info(),
            // Authority to close the mint
            &ctx.accounts.permanent_delegate.to_account_info(),
        )?;

        // Close ATA of immunity
        anchor_spl::token_interface::close_account(CpiContext::new(
            ctx.accounts.token22_program.to_account_info(),
            anchor_spl::token_interface::CloseAccount {
                account: ctx
                    .accounts
                    .token_account_immunity
                    .to_account_info()
                    .clone(),
                destination: ctx.accounts.payer.to_account_info().clone(),
                authority: ctx.accounts.payer.to_account_info().clone(),
            },
        ))?;

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
            "YES".to_string(),
        )?;

        Ok(())
    }
}
