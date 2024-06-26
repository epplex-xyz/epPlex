use crate::*;
pub use anchor_lang::prelude::*;

use anchor_spl::token_interface::{
    burn, close_account, Burn, CloseAccount, TokenAccount as TokenAccountInterface,
};

#[derive(Accounts)]
pub struct MembershipBurn<'info> {
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

    // This PDA receives the burn funds
    #[account(mut)]
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
        seeds = [
            SEED_EPHEMERAL_AUTH
        ],
        bump
    )]
    /// CHECK:
    pub epplex_authority: UncheckedAccount<'info>,

    pub token22_program: Program<'info, Token2022>,
}

impl MembershipBurn<'_> {
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

    /**
     * Closes: ephemeral data, burns token, closes mint, optionally closes ATA
     */
    pub fn burn(ctx: Context<MembershipBurn>) -> Result<()> {
        let seeds: &[&[u8]; 2] = &[SEED_EPHEMERAL_AUTH, &[ctx.bumps.epplex_authority]];
        burn(
            CpiContext::new_with_signer(
                ctx.accounts.token22_program.to_account_info(),
                Burn {
                    mint: ctx.accounts.membership.to_account_info(),
                    from: ctx.accounts.membership_ata.to_account_info(),
                    authority: ctx.accounts.epplex_authority.to_account_info(),
                },
                &[&seeds[..]],
            ),
            1,
        )?;

        // Assuming we have epplex_auth is also the closeMintAuth
        close_account(CpiContext::new_with_signer(
            ctx.accounts.token22_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.membership.to_account_info(),
                destination: ctx.accounts.epplex_treasury.to_account_info(),
                authority: ctx.accounts.epplex_authority.to_account_info(),
            },
            &[&seeds[..]],
        ))?;

        Ok(())
    }
}
