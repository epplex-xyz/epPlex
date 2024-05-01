use crate::*;
use anchor_spl::associated_token::AssociatedToken;
use std::str::FromStr;

#[derive(Accounts)]
#[instruction(params: WnsMemberMintParams)]
pub struct WnsMemberMint<'info> {
    #[account(mut, signer)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub token_account: UncheckedAccount<'info>,

    #[account(
        init,
        seeds = [
            SEED_BURGER_METADATA,
            mint.key().as_ref()
        ],
        payer = payer,
        space = BurgerMetadata::LEN,
        bump,
    )]
    pub token_metadata: Account<'info, BurgerMetadata>,

    #[account(
        mut,
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
    /// CHECK:
    pub member: UncheckedAccount<'info>,

    #[account(
        seeds = [
            wen_new_standard::MANAGER_SEED
        ],
        seeds::program = wen_new_standard::ID.key(),
        bump
    )]
    pub manager: Account<'info, wen_new_standard::Manager>,

    #[account(mut)]
    /// CHECK: This account's data is a buffer of TLV data, will be initialised
    pub extra_metas_account: UncheckedAccount<'info>,

    /*
     * Programs
     */
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
    pub associated_token: Program<'info, AssociatedToken>,
    pub wns: Program<'info, WenNewStandard>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WnsMemberMintParams {
    pub expiry_date: String,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

impl WnsMemberMint<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &WnsMemberMintParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: WnsMemberMintParams) -> Result<()> {
        // Create the burger metadata
        let token_metadata = &mut ctx.accounts.token_metadata;
        **token_metadata = BurgerMetadata::new(ctx.bumps.token_metadata);

        let seeds = &[
            SEED_PROGRAM_DELEGATE,
            &[ctx.accounts.permanent_delegate.bump],
        ];
        let epplex_auth =
            Pubkey::find_program_address(&[epplex_core::SEED_EPHEMERAL_AUTH], &epplex_core::ID).0;

        // 1. Create mint account
        wen_new_standard::cpi::create_mint_account(
            CpiContext::new_with_signer(
                ctx.accounts.wns.to_account_info(),
                wen_new_standard::cpi::accounts::CreateMintAccount {
                    payer: ctx.accounts.payer.to_account_info(),
                    authority: ctx.accounts.permanent_delegate.to_account_info(),
                    receiver: ctx.accounts.payer.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    mint_token_account: ctx.accounts.token_account.to_account_info(),
                    manager: ctx.accounts.manager.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    associated_token_program: ctx.accounts.associated_token.to_account_info(),
                    token_program: ctx.accounts.token22_program.to_account_info(),
                },
                &[&seeds[..]],
            ),
            wen_new_standard::mint::CreateMintAccountArgs {
                name: params.name,
                symbol: params.symbol,
                uri: params.uri,
                permanent_delegate: Some(epplex_auth),
            },
        )?;

        // 2. Add mint to group
        wen_new_standard::cpi::add_mint_to_group(CpiContext::new_with_signer(
            ctx.accounts.wns.to_account_info(),
            wen_new_standard::cpi::accounts::AddGroup {
                payer: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.permanent_delegate.to_account_info(),
                group: ctx.accounts.group.to_account_info(),
                member: ctx.accounts.member.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                manager: ctx.accounts.manager.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token22_program.to_account_info(),
            },
            &[&seeds[..]],
        ))?;

        // 3. Add royalties
        wen_new_standard::cpi::add_royalties(
            CpiContext::new_with_signer(
                ctx.accounts.wns.to_account_info(),
                wen_new_standard::cpi::accounts::AddRoyalties {
                    payer: ctx.accounts.payer.to_account_info(),
                    authority: ctx.accounts.permanent_delegate.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    extra_metas_account: ctx.accounts.extra_metas_account.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token22_program.to_account_info(),
                },
                &[&seeds[..]],
            ),
            wen_new_standard::mint::UpdateRoyaltiesArgs {
                royalty_basis_points: ROYALTY_BASIS_POINTS,
                creators: vec![wen_new_standard::CreatorWithShare {
                    address: Pubkey::from_str(ROYALTY_ADDRESS).unwrap(),
                    share: ROYALTY_SHARE,
                }],
            },
        )?;

        // 4. Add other metadata
        wen_new_standard::cpi::add_metadata(
            CpiContext::new_with_signer(
                ctx.accounts.wns.to_account_info(),
                wen_new_standard::cpi::accounts::AddMetadata {
                    payer: ctx.accounts.payer.to_account_info(),
                    authority: ctx.accounts.permanent_delegate.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    token_program: ctx.accounts.token22_program.to_account_info(),
                },
                &[&seeds[..]],
            ),
            generate_metadata2(params.expiry_date),
        )?;

        Ok(())
    }
}
