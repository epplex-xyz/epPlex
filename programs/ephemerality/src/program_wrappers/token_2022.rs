use anchor_lang::prelude::*;

#[derive(Clone)]
pub struct Token2022;

impl Id for Token2022 {
    fn id() -> Pubkey {
        spl_token_2022::ID
    }
}
