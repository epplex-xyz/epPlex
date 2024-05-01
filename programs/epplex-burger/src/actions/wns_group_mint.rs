use crate::*;
use anchor_spl::associated_token::AssociatedToken;

#[derive(Accounts)]
#[instruction(params: WnsGroupMintParams)]
pub struct WnsGroupMint<'info> {
    #[account(mut, signer)]
    /// CHECK
    pub group_mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub token_account: UncheckedAccount<'info>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE
        ],
        bump = permanent_delegate.bump
    )]
    pub permanent_delegate: Account<'info, ProgramDelegate>,

    #[account(
        mut,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    /*
     * WNS Accounts
     */
    #[account(mut)]
    /// CHECK:
    pub group: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: This account's data is a buffer of TLV data, will be initialised
    pub extra_metas_account: UncheckedAccount<'info>,

    #[account(
        seeds = [
            wen_new_standard::MANAGER_SEED
        ],
        seeds::program = wen_new_standard::ID.key(),
        bump
    )]
    pub manager: Account<'info, wen_new_standard::Manager>,

    #[account(mut)]
    /// CHECK:
    pub distribution_account: UncheckedAccount<'info>,

    /*
     * Programs
     */
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub wns: Program<'info, WenNewStandard>,
    pub royalty_program: Program<'info, WenRoyaltyDistribution>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WnsGroupMintParams {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub max_size: u32,
    pub payment_mint: Pubkey,
}

impl WnsGroupMint<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &WnsGroupMintParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: WnsGroupMintParams) -> Result<()> {
        let seeds = &[
            SEED_PROGRAM_DELEGATE,
            &[ctx.accounts.permanent_delegate.bump],
        ];

        // 1. Create group account
        wen_new_standard::cpi::create_group_account(
            CpiContext::new_with_signer(
                ctx.accounts.wns.to_account_info(),
                wen_new_standard::cpi::accounts::CreateGroupAccount {
                    payer: ctx.accounts.payer.to_account_info(),
                    authority: ctx.accounts.permanent_delegate.to_account_info(),
                    receiver: ctx.accounts.payer.to_account_info(),
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

        // 2. Create distribution account from royalties program
        wen_royalty_distribution::cpi::initialize_distribution(
            CpiContext::new(
                ctx.accounts.royalty_program.to_account_info(),
                wen_royalty_distribution::cpi::accounts::InitializeDistribution {
                    payer: ctx.accounts.payer.to_account_info(),
                    group_mint: ctx.accounts.group_mint.to_account_info(),
                    distribution_account: ctx.accounts.distribution_account.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
            ),
            params.payment_mint,
        )?;

        Ok(())
    }
}
