use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenGameFreezeParams)]
pub struct TokenGameFreeze<'info> {
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
        seeds::program = wen_new_standard::ID,
        bump,
    )]
    pub group_member: Account<'info, wen_new_standard::TokenGroupMember>,

    #[account(
        seeds = [
            SEED_GAME_CONFIG
        ],
        bump = game_config.bump,
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

    // WNS  programs
    #[account(
        seeds = [
            wen_new_standard::MANAGER_SEED
        ],
        seeds::program = wen_new_standard::ID,
        bump
    )]
    pub manager: Account<'info, wen_new_standard::Manager>,

    pub token22_program: Program<'info, Token2022>,



}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenGameFreezeParams {
    pub message: String,
}

impl TokenGameFreeze<'_> {
    pub fn validate(&self, ctx: &Context<Self>, params: &TokenGameFreezeParams) -> Result<()> {
        self.game_config
            .check_valid_collection(&self.group_member, self.mint.key())?;

        // Check that the game is in progress
        // self.game_config
        //     .assert_game_status(GameStatus::InProgress)?;

        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: TokenGameFreezeParams) -> Result<()> {
        let seeds = &[SEED_PROGRAM_DELEGATE, &[ctx.accounts.update_authority.bump]];

        wen_new_standard::cpi::freeze_mint_account(
            CpiContext::new_with_signer(
                ctx.accounts.wns.to_account_info(),
                wen_new_standard::cpi::accounts::FreezeDelegatedAccount {
                    payer: ctx.accounts.payer.to_account_info(),
                    user: ctx.accounts.payer.to_account_info(),
                    delegate_authority: ctx.accounts.payer.to_account_info(),
                    group: ctx.accounts.group.to_account_info(),
                    mint: ctx.accounts.group_mint.to_account_info(),
                    mint_token_account: ctx.accounts.token_account.to_account_info(),
                    manager: ctx.accounts.manager.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    associated_token_program: ctx.accounts.associated_token.to_account_info(),
                    token_program: ctx.accounts.token22_program.to_account_info(),
                },
                &[&seeds[..]],
            ),
            wen_new_standard::CreateGroupAccountArgs {
                name: params.name,
                symbol: params.symbol,
                uri: params.uri,
                max_size: params.max_size,
            },
        )?;

        Ok(())
    }
}
