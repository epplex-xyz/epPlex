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
            SEED_BURGER_METADATA,
            mint.key().as_ref()
        ],
        bump = token_metadata.bump
    )]
    pub token_metadata: Account<'info, BurgerMetadata>,

    // prolly need a game state = that contains game-master as well
    // Only game master can handle this
    #[account(
        address = VAULT_PUBKEY
    )]
    pub payer: Signer<'info>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = update_authority.bump
    )]
    pub update_authority: Account<'info, ProgramDelegate>,

    pub token22_program: Program<'info, Token2022>,
    pub token_program: Program<'info, Token>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenGameResetParams {}

impl TokenGameReset<'_> {
    pub fn validate(&self, ctx: &Context<Self>, _params: &TokenGameResetParams) -> Result<()> {
        // TODO need to validate on game state
        let game_state = fetch_metadata_field(
            GAME_STATE,
            &ctx.accounts.mint.to_account_info()
        )?;

        if !game_state.is_empty() {
            return err!(BurgerError::GameStateMustNotBeEmpty);
        }

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenGameResetParams) -> Result<()> {
        let seeds = &[SEED_PROGRAM_DELEGATE, &[ctx.accounts.update_authority.bump]];
        epplex_shared::update_token_metadata_signed(
            &ctx.accounts.token22_program.key(),
            &ctx.accounts.mint.to_account_info(),
            &ctx.accounts.update_authority.to_account_info(), // the program permanent delegate
            &[&seeds[..]],
            spl_token_metadata_interface::state::Field::Key(GAME_STATE.to_string()),
            "".to_string()
        )?;

        Ok(())
    }
}
