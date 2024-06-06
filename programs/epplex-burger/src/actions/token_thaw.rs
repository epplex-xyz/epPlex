use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenThawParams)]
pub struct TokenThaw<'info> {
    #[account(
        mut,
        mint::token_program = token22_program.key(),
        constraint = mint.decimals == 0,
        constraint = mint.supply == 1,
    )]
    pub mint: Box<InterfaceAccount<'info, MintInterface>>,

    #[account(
        mut,
        token::mint = mint,
        token::authority = user,
        token::token_program = token22_program.key(),
    )]
    pub token_account: Box<InterfaceAccount<'info, TokenAccountInterface>>, // Used to verify owner

    #[account(
        mut,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    // Owner of token account
    #[account()]
    /// CHECK: can be any account
    pub user: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = authority.bump
    )]
    pub authority: Account<'info, ProgramDelegate>,

    // WNS  programs
    // #[account(
    //     seeds = [
    //         wen_new_standard::MANAGER_SEED
    //     ],
    //     seeds::program = wen_new_standard::ID,
    //     bump
    // )]
    // pub manager: Account<'info, wen_new_standard::Manager>,
    #[account()]
    /// CHECK: cpi checks
    pub manager: UncheckedAccount<'info>,


    pub token22_program: Program<'info, Token2022>,
    pub wns: Program<'info, WenNewStandard>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenThawParams {}

impl TokenThaw<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenThawParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: TokenThawParams) -> Result<()> {
        let seeds = &[SEED_PROGRAM_DELEGATE, &[ctx.accounts.authority.bump]];
        let signers_seeds = &[&seeds[..]];

        wen_new_standard::instructions::ThawMintAccountCpi::new(
            &ctx.accounts.wns.to_account_info(),
            wen_new_standard::instructions::ThawMintAccountCpiAccounts{
                user: &ctx.accounts.user.to_account_info(),
                delegate_authority: &ctx.accounts.authority.to_account_info(),
                mint: &ctx.accounts.mint.to_account_info(),
                mint_token_account: &ctx.accounts.token_account.to_account_info(),
                manager: &ctx.accounts.manager.to_account_info(),
                token_program: &ctx.accounts.token22_program.to_account_info(),
            }
        ).invoke_signed(signers_seeds)?;

        Ok(())
    }
}
