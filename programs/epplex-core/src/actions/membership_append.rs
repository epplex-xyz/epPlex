use crate::*;
pub use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MembershipAppend<'info> {
    // This is the T22 NFT
    #[account(mut)]
    /// CHECK
    pub membership: UncheckedAccount<'info>,

    #[account(
        seeds = [
            SEED_EPHEMERAL_RULE,
            rule.seed.to_le_bytes().as_ref()
        ],
        bump,
    )]
    pub rule: Account<'info, EphemeralRule>,

    // Needs to be here since rule_creator needs to sign
    #[account(
        mut,
        constraint = rule_creator.key() == rule.rule_creator
            @EphemeralityError::EscalatedAuthority,
    )]
    pub rule_creator: Signer<'info>,

    #[account(
        init,
        payer = payer,
        space = EphemeralData::INIT_SPACE,
        seeds = [
            SEED_EPHEMERAL_DATA,
            membership.key().as_ref()
        ],
        bump,
    )]
    pub data: Account<'info, EphemeralData>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

impl MembershipAppend<'_> {
    pub fn append(&mut self, time: i64, bumps: MembershipAppendBumps) -> Result<()> {
        self.data.set_inner(EphemeralData {
            bump: bumps.data,
            mint: self.membership.key(),
            rule: self.rule.key(),
            expiry_time: Clock::get()?.unix_timestamp + time,
        });

        Ok(())
    }
}
