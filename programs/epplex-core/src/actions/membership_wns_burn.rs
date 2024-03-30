use crate::*;
pub use anchor_lang::prelude::*;

use anchor_spl::token_interface::TokenAccount as TokenAccountInterface;
use spl_token_2022::onchain::invoke_transfer_checked;

#[derive(Clone)]
pub struct WenNewStandard;

impl Id for WenNewStandard {
    fn id() -> Pubkey {
        wen_new_standard::ID
    }
}

#[derive(Clone)]
pub struct WenRoyaltyDistribution;

impl Id for WenRoyaltyDistribution {
    fn id() -> Pubkey {
        wen_royalty_distribution::ID
    }
}

#[derive(Accounts)]
pub struct MembershipWnsBurn<'info> {
    // This is the T22 NFT
    #[account(mut)]
    /// CHECK
    pub membership: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: to be initialised
    pub membership_ata: UncheckedAccount<'info>,

    #[account(
        mut,
        token::mint = membership.key(),
        token::token_program = token22_program.key(),
    )]
    pub source_ata: Box<InterfaceAccount<'info, TokenAccountInterface>>,

    // Receives the burn funds
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub burner: Signer<'info>,

    #[account(mut)]
    pub epplex_treasury: SystemAccount<'info>,

    #[account(
        seeds = [
            SEED_EPHEMERAL_RULE,
            rule.seed.to_le_bytes().as_ref()
        ],
        bump = rule.bump,
    )]
    pub rule: Account<'info, EphemeralRule>,

    #[account(
        mut,
        close = epplex_treasury,
        seeds = [
            SEED_EPHEMERAL_DATA,
            membership.key().as_ref()
        ],
        bump,
    )]
    pub data: Account<'info, EphemeralData>,

    #[account(
        mut,
        seeds = [
            SEED_EPHEMERAL_AUTH
        ],
        bump
    )]
    /// CHECK:
    pub epplex_authority: UncheckedAccount<'info>,

    /*
       WNS stuff
    */
    // Transfer Hook Accounts
    #[account()]
    /// CHECK: no need to check it out, the invoke_transfer will check for us
    pub metas_account_list: AccountInfo<'info>,

    // For burning
    #[account(
        seeds = [wen_new_standard::MANAGER_SEED],
        seeds::program = wen_new_standard::ID,
        bump
    )]
    pub manager: Account<'info, wen_new_standard::Manager>,

    // Approve
    #[account(mut)]
    /// CHECK: initialized token account or unitialized token account, checks in cpi
    pub approve_account: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: initialized token account or unitialized token account, checks in cpi
    pub distribution_token_account: UncheckedAccount<'info>,
    #[account(mut)]
    /// CHECK: cpi checks
    pub distribution_account: UncheckedAccount<'info>,
    #[account()]
    /// CHECK: This account can be any mint or SOL
    pub payment_mint: UncheckedAccount<'info>,

    pub wrd: Program<'info, WenRoyaltyDistribution>,
    pub wns: Program<'info, WenNewStandard>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token22_program: Program<'info, Token2022>,
}

impl MembershipWnsBurn<'_> {
    pub fn validate(&self, _ctx: &Context<Self>) -> Result<()> {
        // Rule Creator maintains burn auth
        if self.burner.key() == self.rule.rule_creator {
            return Ok(());
        }

        // Anyone can execute if time has expired
        require!(
            self.data.expiry_time + GRACE_PERIOD < Clock::get()?.unix_timestamp,
            EphemeralityError::NotExpired
        );

        Ok(())
    }

    pub fn burn(ctx: Context<MembershipWnsBurn>) -> Result<()> {
        let seeds: &[&[u8]; 2] = &[SEED_EPHEMERAL_AUTH, &[ctx.bumps.epplex_authority]];

        // 0. Create pda on ATA epplex_auth
        create(CpiContext::new_with_signer(
            ctx.accounts.associated_token_program.to_account_info(),
            Create {
                payer: ctx.accounts.payer.to_account_info(), // payer
                associated_token: ctx.accounts.membership_ata.to_account_info(),
                mint: ctx.accounts.membership.to_account_info(),
                authority: ctx.accounts.epplex_authority.to_account_info(), // owner
                system_program: ctx.accounts.system_program.to_account_info(),
                token_program: ctx.accounts.token22_program.to_account_info(),
            },
            &[&seeds[..]],
        ))?;

        // 1. Approve transfer
        wen_new_standard::cpi::approve_transfer(
            CpiContext::new_with_signer(
                ctx.accounts.wns.to_account_info(),
                wen_new_standard::cpi::accounts::ApproveTransfer {
                    payer: ctx.accounts.payer.to_account_info(),
                    authority: ctx.accounts.epplex_authority.to_account_info(),
                    mint: ctx.accounts.membership.to_account_info(),
                    approve_account: ctx.accounts.approve_account.to_account_info(),
                    payment_mint: ctx.accounts.payment_mint.to_account_info(),
                    distribution_token_account: ctx
                        .accounts
                        .distribution_token_account
                        .to_account_info(),
                    authority_token_account: ctx.accounts.membership_ata.to_account_info(),
                    distribution_account: ctx.accounts.distribution_account.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    distribution_program: ctx.accounts.wrd.to_account_info(),
                    token_program: ctx.accounts.token22_program.to_account_info(),
                    associated_token_program: ctx
                        .accounts
                        .associated_token_program
                        .to_account_info(),
                },
                &[&seeds[..]],
            ),
            1,
        )?;

        // 2. Transfer with transfer hook
        invoke_transfer_checked(
            &ctx.accounts.token22_program.key(),
            ctx.accounts.source_ata.to_account_info(),
            ctx.accounts.membership.to_account_info(),
            ctx.accounts.membership_ata.to_account_info(),
            ctx.accounts.epplex_authority.to_account_info(),
            &[
                ctx.accounts.approve_account.to_account_info(),
                ctx.accounts.wns.to_account_info(),
                ctx.accounts.metas_account_list.to_account_info(),
            ],
            1u64,
            0u8,
            &[&seeds[..]],
        )?;

        // 3. Burn token, close mint and close token account
        wen_new_standard::cpi::burn_mint_account(CpiContext::new_with_signer(
            ctx.accounts.wns.to_account_info(),
            wen_new_standard::cpi::accounts::BurnMintAccount {
                payer: ctx.accounts.payer.to_account_info(),
                user: ctx.accounts.epplex_authority.to_account_info(),
                mint: ctx.accounts.membership.to_account_info(),
                mint_token_account: ctx.accounts.membership_ata.to_account_info(),
                manager: ctx.accounts.manager.to_account_info(),
                token_program: ctx.accounts.token22_program.to_account_info(),
            },
            &[&seeds[..]],
        ))?;

        Ok(())
    }
}
