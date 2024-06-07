use crate::*;
use anchor_spl::{
    token_2022::spl_token_2022::extension::BaseStateWithExtensions,
    token_interface::spl_token_metadata_interface::borsh::BorshDeserialize,
};
// use spl_token_metadata_interface::borsh::BorshDeserialize;

#[allow(clippy::ptr_arg)]
pub fn get_value_by_key<'a>(key: &'a str, list: &'a Vec<(String, String)>) -> Option<&'a String> {
    // Use iter() to create an iterator over the vector
    // Use find() to search for the key-value pair where the key matches
    let result = list.iter().find(|&(k, _)| k == key);

    // If a matching key-value pair is found, return the associated value
    match result {
        Some((_, v)) => Some(v),
        None => None,
    }
}

// Fetch a particular value
pub fn fetch_metadata_field(field: &str, mint_account: &AccountInfo) -> Result<String> {
    // let buffer = mint_account.to_account_info();
    let value;

    // Fetch the expiry date
    {
        // Scoping because borrowing later
        let mint_data = mint_account.try_borrow_data()?;
        let state = anchor_spl::token_2022::spl_token_2022::extension::StateWithExtensions::<
            anchor_spl::token_2022::spl_token_2022::state::Mint,
        >::unpack(&mint_data)?;
        let metadata_bytes = state.get_extension_bytes::<anchor_spl::token_interface::spl_token_metadata_interface::state::TokenMetadata>().unwrap();
        let fetched_metadata = anchor_spl::token_interface::spl_token_metadata_interface::state::TokenMetadata::try_from_slice(metadata_bytes)?;

        // let temp = get_value_by_key(EXPIRY_FIELD, &fetched_metadata.additional_metadata)?;
        // expiry_date = temp.parse::<i64>()?;
        value = get_value_by_key(field, &fetched_metadata.additional_metadata)
            .unwrap()
            .clone()
        // .ok_or(err!(BurgerError::FieldDoesNotExist))?;
    }

    Ok(value)
}
