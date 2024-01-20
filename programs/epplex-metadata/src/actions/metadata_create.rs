use crate::*;

#[derive(Accounts)]
pub struct MetadataCreate<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // TODO: is unchecked account correct?
    #[account(mut, signer)]
    /// CHECK
    pub mint: UncheckedAccount<'info>,


    // TODO how to get exact space of this?
    #[account(
        init,
        seeds = [
            SEED_TOKEN_METADATA,
            mint.key().as_ref()
        ],
        payer = payer,
        space = TokenMetadata::LEN,
        bump,
    )]
    pub token_metadata: Account<'info, TokenMetadata>,

    pub system_program: Program<'info, System>
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct MetadataCreateParams {
    pub destroy_timestamp: i64,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

impl MetadataCreate<'_> {
    pub fn validate(&self, _ctx: &Context<Self>, _params: &MetadataCreateParams) -> Result<()> {
        Ok(())
    }

    pub fn actuate(ctx: Context<Self>, params: MetadataCreateParams) -> Result<()> {
        let token_metadata = &mut ctx.accounts.token_metadata;
        **token_metadata = TokenMetadata {
            // TODO this update auth seems incorrect
            update_authority: self::ID,
            name: params.name,
            symbol: params.symbol,
            uri: params.uri,
            mint: *ctx.accounts.mint.key,
            additional_metadata: vec![
                [EXPIRY_FIELD.to_string(), params.destroy_timestamp.to_string()]
            ]
        };

        Ok(())
    }
}