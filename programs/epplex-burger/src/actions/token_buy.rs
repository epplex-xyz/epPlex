use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenBuyParams)]
pub struct TokenBuy<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenBuyParams {

}

impl TokenBuy<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &TokenBuyParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: TokenBuyParams) -> Result<()> {
        // simply need to call launchpad with its own permanent delegate
        Ok(())
    }

}
