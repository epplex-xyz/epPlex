use crate::*;
pub use anchor_lang::prelude::*;

use anchor_spl::{
    token_2022::{burn, close_account, Burn, CloseAccount},
    token_interface::TokenAccount as TokenAccountInterface,
};

#[derive(Clone)]
pub struct WenNewStandard;

impl Id for WenNewStandard {
    fn id() -> Pubkey {
        wen_new_standard::ID
    }
}

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
        bump,
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

    #[account(
        seeds = [wen_new_standard::MANAGER_SEED],
        seeds::program = wen_new_standard::ID,
        bump
    )]
    pub manager: Option<Account<'info, wen_new_standard::Manager>>,

    pub wns: Option<Program<'info, WenNewStandard>>,
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
    pub fn burn<'info>(ctx: Context<'_, '_, '_, 'info, MembershipBurn<'info>>) -> Result<()> {
        // let burner1 = ctx.accounts.burner.to_account_info().clone();
        // let burner2 = ctx.accounts.burner.to_account_info().clone();
        // let token_program = ctx.accounts.token22_program.to_account_info().clone();

        // let token_account = ctx.accounts.membership_ata.to_account_info().clone();

        match ctx.accounts.wns {
            Some(_) => Self::burn_wns(ctx)?,
            None => Self::burn_standard(ctx)?,
        }

        // let state =
        //     spl_token_2022::state::Account::unpack_from_slice(&token_account.try_borrow_data()?)?;
        // if state.owner == burner1.key() {
        //     close_account(CpiContext::new(
        //         token_program,
        //         CloseAccount {
        //             account: token_account,
        //             destination: burner1.clone(),
        //             authority: burner2,
        //         },
        //     ))?;
        // }

        Ok(())
    }

    fn burn_standard<'info>(ctx: Context<'_, '_, '_, 'info, MembershipBurn<'info>>) -> Result<()> {
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

    fn burn_wns<'info>(ctx: Context<'_, '_, '_, 'info, MembershipBurn<'info>>) -> Result<()> {
        let seeds: &[&[u8]; 2] = &[SEED_EPHEMERAL_AUTH, &[ctx.bumps.epplex_authority]];

        let wns = ctx.accounts.wns.clone().unwrap();
        let manager = ctx.accounts.manager.clone().unwrap();

        wen_new_standard::cpi::burn_mint_account(CpiContext::new_with_signer(
            wns.to_account_info(),
            wen_new_standard::cpi::accounts::BurnMintAccount {
                payer: ctx.accounts.burner.to_account_info(),
                user: ctx.accounts.epplex_authority.to_account_info(),
                mint: ctx.accounts.membership.to_account_info(),
                mint_token_account: ctx.accounts.membership_ata.to_account_info(),
                manager: manager.to_account_info(),
                token_program: ctx.accounts.token22_program.to_account_info(),
            },
            &[&seeds[..]],
        ))?;

        Ok(())
    }
}
