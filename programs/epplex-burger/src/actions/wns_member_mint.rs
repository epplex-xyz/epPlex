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
    pub expiry_date: Option<String>,
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub creators: Option<Vec<wen_new_standard::types::CreatorWithShare>>,
    pub add_permanent_delegate: bool,
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
        let signers_seeds = &[&seeds[..]];
        let epplex_auth =
            Pubkey::find_program_address(&[epplex_core::SEED_EPHEMERAL_AUTH], &epplex_core::ID).0;

        // 1. Create mint account
        wen_new_standard::instructions::CreateMintAccountCpi::new(
            &ctx.accounts.wns.to_account_info(),
            wen_new_standard::instructions::CreateMintAccountCpiAccounts{
                payer: &ctx.accounts.payer.to_account_info(),
                authority: &ctx.accounts.permanent_delegate.to_account_info(),
                receiver: &ctx.accounts.payer.to_account_info(),
                mint: &ctx.accounts.mint.to_account_info(),
                mint_token_account: &ctx.accounts.token_account.to_account_info(),
                manager: &ctx.accounts.manager.to_account_info(),
                system_program: &ctx.accounts.system_program.to_account_info(),
                associated_token_program: &ctx.accounts.associated_token.to_account_info(),
                token_program: &ctx.accounts.token22_program.to_account_info(),
            },
            wen_new_standard::instructions::CreateMintAccountInstructionArgs {
                args: wen_new_standard::types::CreateMintAccountArgs {
                    name: params.name,
                    symbol: params.symbol,
                    uri: params.uri,
                    permanent_delegate: if params.add_permanent_delegate { Some(epplex_auth) } else { None }
                }
            }
        ).invoke_signed(signers_seeds)?;

        // 2. Add mint to group
        wen_new_standard::instructions::AddMintToGroupCpi::new(
            &ctx.accounts.wns.to_account_info(),
            wen_new_standard::instructions::AddMintToGroupCpiAccounts{
                payer: &ctx.accounts.payer.to_account_info(),
                authority: &ctx.accounts.permanent_delegate.to_account_info(),
                group: &ctx.accounts.group.to_account_info(),
                member: &ctx.accounts.member.to_account_info(),
                mint: &ctx.accounts.mint.to_account_info(),
                manager: &ctx.accounts.manager.to_account_info(),
                system_program: &ctx.accounts.system_program.to_account_info(),
                token_program: &ctx.accounts.token22_program.to_account_info(),
            }
        ).invoke_signed(signers_seeds)?;

        let mut creators = vec![wen_new_standard::types::CreatorWithShare {
            address: Pubkey::from_str(ROYALTY_ADDRESS).unwrap(),
            share: ROYALTY_SHARE,
        }];
        if params.creators.is_some() {
            creators = params.creators.unwrap();
        }

        // 3. Add royalties
        wen_new_standard::instructions::AddRoyaltiesCpi::new(
            &ctx.accounts.wns.to_account_info(),
            wen_new_standard::instructions::AddRoyaltiesCpiAccounts {
                payer: &ctx.accounts.payer.to_account_info(),
                authority: &ctx.accounts.permanent_delegate.to_account_info(),
                mint: &ctx.accounts.mint.to_account_info(),
                extra_metas_account: &ctx.accounts.extra_metas_account.to_account_info(),
                system_program: &ctx.accounts.system_program.to_account_info(),
                token_program: &ctx.accounts.token22_program.to_account_info(),
            },
            wen_new_standard::instructions::AddRoyaltiesInstructionArgs {
                args: wen_new_standard::types::UpdateRoyaltiesArgs {
                    royalty_basis_points: ROYALTY_BASIS_POINTS,
                    creators,
                }
            }
        ).invoke_signed(signers_seeds)?;


        let mut metadata = vec![];
        if params.expiry_date.is_some() {
            metadata = generate_metadata2(params.expiry_date.unwrap())
        }
        wen_new_standard::instructions::AddMetadataCpi::new(
            &ctx.accounts.wns.to_account_info(),
            wen_new_standard::instructions::AddMetadataCpiAccounts{
                payer: &ctx.accounts.payer.to_account_info(),
                authority: &ctx.accounts.permanent_delegate.to_account_info(),
                mint: &ctx.accounts.mint.to_account_info(),
                system_program: &ctx.accounts.system_program.to_account_info(),
                token_program: &ctx.accounts.token22_program.to_account_info(),
             },
             wen_new_standard::instructions::AddMetadataInstructionArgs {
                args: metadata
             }
        ).invoke_signed(signers_seeds)?;

        Ok(())
    }
}
