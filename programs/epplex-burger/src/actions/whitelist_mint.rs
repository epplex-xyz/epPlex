use crate::*;

#[derive(Accounts)]
#[instruction(params: WhitelistMintParams)]
pub struct WhitelistMint<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct WhitelistMintParams {

}

impl WhitelistMint<'_> {

    pub fn validate(&self, _ctx: &Context<Self>, _params: &WhitelistMintParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(_ctx: Context<Self>, _params: WhitelistMintParams) -> Result<()> {
        // simply need to call launchpad with its own permanent delegate
        Ok(())
    }

}
