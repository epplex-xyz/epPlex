use crate::*;
pub use anchor_lang::prelude::*;

use anchor_spl::{
    token_2022::{burn, Burn, close_account, CloseAccount, Token2022},
    token_interface::{TokenAccount as TokenAccountInterface},
};
use solana_program::program_pack::Pack;

#[derive(Accounts)]
pub struct MembershipBurn<'info> {
    #[account(mut)]
    pub burner: Signer<'info>,

    #[account(mut)]
    pub epplex: SystemAccount<'info>,

    // This is the T22 NFT
    #[account(mut)]
    /// CHECK
    pub membership: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint = membership.key(),
        token::token_program = token22_program.key(),
    )]
    pub membership_ata: Box<InterfaceAccount<'info, TokenAccountInterface>>,

    #[account(
        seeds = [
            SEED_EPHEMERAL_RULE,
            rule.seed.to_le_bytes().as_ref()
        ],
        bump,
    )]
    pub rule: Account<'info, EphemeralRule>,

    #[account(
        mut,
        close = epplex,
        seeds = [
            SEED_EPHEMERAL_DATA,
            membership.key().as_ref()
        ],
        bump,
    )]
    pub data: Account<'info, EphemeralData>,

    /// CHECK:
    #[account(
        seeds = [
            SEED_EPHEMERAL_AUTH
        ],
        bump
    )]
    pub authority: UncheckedAccount<'info>,

    pub token22_program: Program<'info, Token2022>,

    pub system_program: Program<'info, System>,
}

impl MembershipBurn<'_> {
    pub fn burn(ctx: Context<Self>) -> Result<()> {
        require!(
            ctx.accounts.data.expiry_time + 14 * 3600 < Clock::get()?.unix_timestamp
            || ctx.accounts.burner.key() == ctx.accounts.rule.rule_creator
            , EphemeralityError::NotExpired
        );

        let seeds: &[&[u8]; 2] = &[SEED_EPHEMERAL_AUTH, &[ctx.bumps.authority]];
        burn(
            CpiContext::new_with_signer(
                ctx.accounts.token22_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.membership.to_account_info(),
                    from: ctx.accounts.membership_ata.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
                &[&seeds[..]],
            ),
            1
        )?;

        close_account(
            CpiContext::new(
                ctx.accounts.token22_program.to_account_info(),
                CloseAccount {
                    account: ctx.accounts.membership.to_account_info(),
                    destination: ctx.accounts.epplex.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                }
            )
        )?;

        // Close ATA if owner of ATA is burner
        let token_account = ctx.accounts.membership_ata.to_account_info();
        let state = spl_token_2022::state::Account::unpack_from_slice(
            &token_account.try_borrow_data()?
        )?;
        if state.owner == ctx.accounts.burner.key() {
            anchor_spl::token_interface::close_account(
                CpiContext::new(
                    ctx.accounts.token22_program.to_account_info(),
                    anchor_spl::token_interface::CloseAccount {
                        account: ctx.accounts.membership_ata.to_account_info().clone(),
                        destination: ctx.accounts.burner.to_account_info().clone(),
                        authority: ctx.accounts.burner.to_account_info().clone(),
                    },
                ),
            )?;
        }


        Ok(())
    }
}