use anchor_lang::prelude::*;

#[derive(Clone)]
pub struct AssociatedToken;

impl Id for AssociatedToken {
    fn id() -> Pubkey {
        spl_associated_token_account::ID
    }
}
