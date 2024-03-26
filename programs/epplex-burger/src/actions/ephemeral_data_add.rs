use crate::*;
use epplex_core::program::EpplexCore;

#[derive(Accounts)]
#[instruction(params: EphemeralDataAddParams)]
pub struct EphemeralDataAdd<'info> {
    #[account(
        mut,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    #[account()]
    /// CHECK: this will be checked by the CORE program
    pub nft: UncheckedAccount<'info>,

    #[account()]
    /// CHECK: this will be checked by the CORE program
    pub rule: UncheckedAccount<'info>,

    #[account(
        seeds = [
            SEED_PROGRAM_DELEGATE,
        ],
        bump = rule_creator.bump
    )]
    pub rule_creator: Account<'info, ProgramDelegate>,

    #[account(mut)]
    /// CHECK: this will be checked by the CORE program
    pub data: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub epplex_core: Program<'info, EpplexCore>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct EphemeralDataAddParams {
    pub time: i64,
}

impl EphemeralDataAdd<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &EphemeralDataAddParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: EphemeralDataAddParams) -> Result<()> {
        let seeds = &[SEED_PROGRAM_DELEGATE, &[ctx.accounts.rule_creator.bump]];

        epplex_core::cpi::membership_append(
            CpiContext::new_with_signer(
                ctx.accounts.epplex_core.to_account_info(),
                epplex_core::cpi::accounts::MembershipAppend {
                    membership: ctx.accounts.nft.to_account_info(),
                    rule: ctx.accounts.rule.to_account_info(),
                    rule_creator: ctx.accounts.rule_creator.to_account_info(),
                    data: ctx.accounts.data.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
                &[&seeds[..]],
            ),
            params.time,
        )?;

        Ok(())
    }
}
