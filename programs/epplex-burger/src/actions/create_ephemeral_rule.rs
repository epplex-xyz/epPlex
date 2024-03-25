use crate::*;
use epplex_core::program::EpplexCore;

#[derive(Accounts)]
#[instruction(params: CreateEphemeralRuleParams)]
pub struct CreateEphemeralRule<'info> {
    #[account(
        mut,
        constraint = ADMINS.contains(
            &payer.key()
        ) @ BurgerError::NonOperator
    )]
    pub payer: Signer<'info>,

    #[account(mut)]
    /// CHECK: this will be checked by the CORE program
    pub rule: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
    pub epplex_core: Program<'info, EpplexCore>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateEphemeralRuleParams {
    pub seed: u64,
    pub rule_creator: Pubkey,
    pub renewal_price: u64,
    pub treasury: Pubkey,
}

impl CreateEphemeralRule<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &CreateEphemeralRuleParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: CreateEphemeralRuleParams) -> Result<()> {
        require_eq!(params.rule_creator, Pubkey::find_program_address(&[SEED_PROGRAM_DELEGATE], &ID).0, BurgerError::WrongRuleCreator);
        
        epplex_core::cpi::rule_create(
            CpiContext::new(
                ctx.accounts.epplex_core.to_account_info(),
                epplex_core::cpi::accounts::RuleManage {
                    signer: ctx.accounts.payer.to_account_info(),
                    rule: ctx.accounts.rule.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
            ),
            epplex_core::RuleManageParams { 
                seed: params.seed, 
                rule_creator: params.rule_creator, 
                renewal_price: params.renewal_price, 
                treasury: params.treasury 
            },
        )?;

        Ok(())
    }
}
