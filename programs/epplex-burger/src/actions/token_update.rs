use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenUpdateParams)]
pub struct TokenUpdate<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        mut,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
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
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorDeserialize, AnchorSerialize)]
pub struct AddMetadataArgs {
    pub field: String,
    pub value: String,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenUpdateParams {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub uri: Option<String>,
    pub additional_metadata: Option<AddMetadataArgs>,
    pub remove_key: Option<String>,
}

impl TokenUpdate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenUpdateParams) -> Result<()> {
        // This prolly needs to be added once we have DAO updates

        // self.game_config
        // .check_valid_collection(&self.group_member, self.mint.key())?

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: TokenUpdateParams) -> Result<()> {
        let seeds = &[SEED_PROGRAM_DELEGATE, &[ctx.accounts.update_authority.bump]];

        if params.name.is_some() {
            epplex_shared::update_token_metadata_signed(
                &ctx.accounts.token22_program.key(),
                &ctx.accounts.mint.to_account_info(),
                &ctx.accounts.update_authority.to_account_info(),
                &[&seeds[..]],
                anchor_spl::token_interface::spl_token_metadata_interface::state::Field::Name,
                params.name.unwrap(),
            )?;
        }

        if params.symbol.is_some() {
            epplex_shared::update_token_metadata_signed(
                &ctx.accounts.token22_program.key(),
                &ctx.accounts.mint.to_account_info(),
                &ctx.accounts.update_authority.to_account_info(),
                &[&seeds[..]],
                anchor_spl::token_interface::spl_token_metadata_interface::state::Field::Symbol,
                params.symbol.unwrap(),
            )?;
        }

        if params.uri.is_some() {
            epplex_shared::update_token_metadata_signed(
                &ctx.accounts.token22_program.key(),
                &ctx.accounts.mint.to_account_info(),
                &ctx.accounts.update_authority.to_account_info(),
                &[&seeds[..]],
                anchor_spl::token_interface::spl_token_metadata_interface::state::Field::Uri,
                params.uri.unwrap(),
            )?;
        }

        if params.additional_metadata.is_some() {
            let meta = params.additional_metadata.unwrap();
            epplex_shared::update_token_metadata_signed(
                &ctx.accounts.token22_program.key(),
                &ctx.accounts.mint.to_account_info(),
                &ctx.accounts.update_authority.to_account_info(),
                &[&seeds[..]],
                anchor_spl::token_interface::spl_token_metadata_interface::state::Field::Key(
                    meta.field,
                ),
                meta.value,
            )?;
        }

        if params.remove_key.is_some() {
            let removal_key = params.remove_key.unwrap();
            epplex_shared::remove_token_metadata_signed(
                &ctx.accounts.token22_program.key(),
                &ctx.accounts.mint.to_account_info(),
                &ctx.accounts.update_authority.to_account_info(),
                &[&seeds[..]],
                removal_key,
                false,
            )?;
        }

        epplex_shared::update_account_lamports_to_minimum_balance(
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        )?;

        Ok(())
    }
}
