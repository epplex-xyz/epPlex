use crate::*;
use spl_token_2022::extension::BaseStateWithExtensions;

pub fn get_value_by_key<'a>(key: &'a str, list: &'a Vec<(String, String)>) -> Option<&'a String> {
    // Use iter() to create an iterator over the vector
    // Use find() to search for the key-value pair where the key matches
    let result = list.iter().find(|&(k, _)| k == key);

    // If a matching key-value pair is found, return the associated value
    match result {
        Some(&(_,  ref v)) => Some(v),
        None => None,
    }
}

// pub fn get_value_by_key(key: &str, list: &Vec<(String, String)>) -> Option<&String> {
//     // Use iter() to create an iterator over the vector
//     // Use find() to search for the key-value pair where the key matches
//     let result = list.iter().find(|&(k, _)| k == key);
//
//     // If a matching key-value pair is found, return the associated value
//     match result {
//         Some((_, v)) => Some(v),
//         None => None,
//     }
// }



// fn get_value_by_key<'a>(key: &str, list: &'a Vec<(String, String)>) -> Option<&'a String> {
//     let result = list.iter().find(|&&(ref k, _)| k == key);
//
//     match result {
//         Some(&(_,  v)) => Some(v),
//         None => None,
//     }
// }
// fn get_value_by_key(key: &str, list: &Vec<(String, String)>) -> Option<&String> {
//     list.iter().find(|&&(ref k, _)| k == key).map(|&(_, ref v)| v)
// }

// Fetch a particular value
pub fn fetch_metadata_field(field: &str, mint_account: &AccountInfo) -> Result<String> {
    // let buffer = mint_account.to_account_info();
    let value;

    // Fetch the expiry date
    {
        // Scoping because borrowing later
        let mint_data = mint_account.try_borrow_data()?;
        let state = spl_token_2022::extension::StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;
        let metadata_bytes = state.get_extension_bytes::<TokenMetadata>().unwrap();
        let fetched_metadata = TokenMetadata::try_from_slice(metadata_bytes)?;

        // let temp = get_value_by_key(EXPIRY_FIELD, &fetched_metadata.additional_metadata)?;
        // expiry_date = temp.parse::<i64>()?;
        value = get_value_by_key(field, &fetched_metadata.additional_metadata).unwrap().clone()
            // .ok_or(err!(BurgerError::FieldDoesNotExist))?;
    }

    Ok(value)
}