use crate::*;

#[derive(Accounts)]
#[instruction(params: VoteCastParams)]
pub struct VoteCast<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct VoteCastParams {
    pub message: String
}

impl VoteCast<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &VoteCastParams) -> Result<()> {
        // need to accept the token mint
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: VoteCastParams) -> Result<()> {

        Ok(())
    }

}
