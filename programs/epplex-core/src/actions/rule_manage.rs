use crate::*;

#[derive(Accounts)]
#[instruction(params: RuleManageParams)]
pub struct RuleManage<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = EphemeralRule::INIT_SPACE,
        seeds = [
            SEED_EPHEMERAL_RULE,
            params.seed.to_le_bytes().as_ref()
        ],
        bump,
    )]
    pub rule: Account<'info, EphemeralRule>,

    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct RuleManageParams {
    pub seed: u64,
    pub rule_creator: Pubkey,
    pub renewal_price: u64,
    pub treasury: Pubkey,
}

impl RuleManage<'_> {
    pub fn rule_create(ctx: Context<Self>, params: RuleManageParams) -> Result<()> {
        let rule = &mut ctx.accounts.rule;
        rule.set_inner(
            EphemeralRule {
                bump: ctx.bumps.rule,
                seed: params.seed,
                rule_creator: params.rule_creator,
                renewal_price: params.renewal_price,
                treasury: params.treasury,
            }
        );

        Ok(())
    }

    pub fn rule_modify(ctx: Context<Self>, params: RuleManageParams) -> Result<()> {
        require!(
            ctx.accounts.rule.rule_creator == ctx.accounts.signer.key(),
            EphemeralityError::EscalatedAuthority
        );

        let rule = &mut ctx.accounts.rule;
        rule.rule_creator = params.rule_creator;
        rule.renewal_price = params.renewal_price;
        rule.treasury = params.treasury;

        Ok(())
    }
}