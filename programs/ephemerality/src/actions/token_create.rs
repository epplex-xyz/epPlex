use crate::*;

#[derive(Accounts)]
#[instruction(params: TokenCreateParams)]
pub struct TokenCreate<'info> {
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct TokenCreateParams {}

impl TokenCreate<'_> {
    pub fn validate(&self, ctx: &Context<Self>, params: &TokenCreateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: &TokenCreateParams) -> Result<()> {
        Ok(())
    }
}
