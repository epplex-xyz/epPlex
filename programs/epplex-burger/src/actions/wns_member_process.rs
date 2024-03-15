use crate::*;
use anchor_spl::associated_token::AssociatedToken;
// use std::str::FromStr;

#[derive(Accounts)]
#[instruction(params: WnsMemberProcessParams)]
pub struct WnsMemberProcess<'info> {
    #[account(mut)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK
    pub token_account: UncheckedAccount<'info>,

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
pub struct WnsMemberProcessParams {}

impl WnsMemberProcess<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &WnsMemberProcessParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, _params: WnsMemberProcessParams) -> Result<()> {
        let seeds = &[
            SEED_PROGRAM_DELEGATE,
            &[ctx.accounts.permanent_delegate.bump],
        ];

        // let mint = ctx.accounts.mint.to_account_info();
        // let mint_no_signer = AccountInfo {
        //     key: mint.key,
        //     is_signer: true,
        //     is_writable: true,
        //     lamports: mint.lamports,
        //     data: mint.data,
        //     owner: mint.owner,
        //     executable: mint.executable,
        //     rent_epoch: mint.rent_epoch
        // };
        // AccountInfo::new(key, is_signer, is_writable, lamports, data, owner, executable, rent_epoch)

        msg! {"here0 {:?}", ctx.accounts.mint.to_account_info()}
        // msg!{"here1 {:?}", mint_no_signer}
        // 2. Add mint to group
        wen_new_standard::cpi::add_mint_to_group(CpiContext::new_with_signer(
            ctx.accounts.wns.to_account_info(),
            wen_new_standard::cpi::accounts::AddGroup {
                payer: ctx.accounts.payer.to_account_info(),
                authority: ctx.accounts.permanent_delegate.to_account_info(),
                group: ctx.accounts.group.to_account_info(),
                member: ctx.accounts.member.to_account_info(),
                // mint: mint_no_signer,
                mint: ctx.accounts.mint.to_account_info(),
                manager: ctx.accounts.manager.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token22_program.to_account_info(),
            },
            &[&seeds[..]],
        ))?;

        // let ix = wen_new_standard::instructions::mint::group::(
        //     &wen_new_standard::ID,
        //     &metadata.key(),
        //     &update_authority.key(),
        //     field,
        //     value,
        // );

        // let account_infos: Vec<AccountInfo> = vec![
        //     metadata.to_account_info(),
        //     update_authority.to_account_info(),
        // ];

        // solana_program::program::invoke_signed(&ix, &account_infos[..], signer_seeds)?;

        msg! {"here"}

        // 3. Add royalties
        // wen_new_standard::cpi::add_royalties(
        //     CpiContext::new_with_signer(
        //         ctx.accounts.wns.to_account_info(),
        //         wen_new_standard::cpi::accounts::AddRoyalties {
        //             payer: ctx.accounts.payer.to_account_info(),
        //             authority: ctx.accounts.permanent_delegate.to_account_info(),
        //             mint: ctx.accounts.mint.to_account_info(),
        //             extra_metas_account: ctx.accounts.extra_metas_account.to_account_info(),
        //             system_program: ctx.accounts.system_program.to_account_info(),
        //             token_program: ctx.accounts.token22_program.to_account_info(),
        //         },
        //         &[&seeds[..]],
        //     ),
        //     wen_new_standard::mint::UpdateRoyaltiesArgs {
        //         royalty_basis_points: ROYALTY_BASIS_POINTS,
        //         creators: vec![wen_new_standard::CreatorWithShare {
        //             address: Pubkey::from_str(ROYALTY_ADDRESS).unwrap(),
        //             share: ROYALTY_SHARE,
        //         }],
        //     },
        // )?;

        // msg!{"here2"}

        // // 4. Add other metadata
        // wen_new_standard::cpi::add_metadata(
        //     CpiContext::new_with_signer(
        //         ctx.accounts.wns.to_account_info(),
        //         wen_new_standard::cpi::accounts::AddMetadata {
        //             payer: ctx.accounts.payer.to_account_info(),
        //             authority: ctx.accounts.permanent_delegate.to_account_info(),
        //             mint: ctx.accounts.mint.to_account_info(),
        //             system_program: ctx.accounts.system_program.to_account_info(),
        //             token_program: ctx.accounts.token22_program.to_account_info(),
        //         },
        //         &[&seeds[..]],
        //     ),
        //     generate_metadata2(params.expiry_date),
        // )?;

        Ok(())
    }
}
