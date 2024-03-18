use crate::*;

pub fn check_has_expired(mint: &AccountInfo) -> Result<()> {
    let expiry_date_string = fetch_metadata_field(EXPIRY_FIELD, mint)?;
    let expiry_date = expiry_date_string.parse::<i64>().unwrap();

    // Cannot exceed expiry
    let now: i64 = Clock::get().unwrap().unix_timestamp;
    msg!("Destroy timestamp: {:?}, now {:?}", expiry_date, now);
    if now < expiry_date {
        return err!(BurgerError::NotYetExpired);
    }

    Ok(())
}

pub fn assert_metadata_fields_empty(mint: &AccountInfo) -> Result<()> {
    let game_state = fetch_metadata_field(GAME_STATE, mint)?;
    let vote_ts = fetch_metadata_field(VOTING_TIMESTAMP, mint)?;

    if !game_state.is_empty() {
        return err!(BurgerError::ExpectedEmptyField);
    }

    if !vote_ts.is_empty() {
        return err!(BurgerError::ExpectedEmptyField);
    }

    Ok(())
}

/// check that the metadata fields are not empty or filled with initial default values
pub fn assert_metadata_fields_filled(mint: &AccountInfo) -> Result<()> {
    let game_state = fetch_metadata_field(GAME_STATE, mint)?;

    if game_state.is_empty() || game_state == GAME_STATE_PLACEHOLDER {
        msg!("game status {:?}", game_state);
        // default game state means user hasn't participated in the game
        return err!(BurgerError::InvalidGameState);
    }

    let voting_ts = fetch_metadata_field(VOTING_TIMESTAMP, mint)?;
    if voting_ts.is_empty() || voting_ts == VOTING_TIMESTAMP_PLACEHOLDER {
        return err!(BurgerError::InvalidExpiryTS);
    }

    Ok(())
}

pub fn check_mint_expiry_ts(mint: &AccountInfo) -> Result<()> {
    let expiry_ts = fetch_metadata_field(EXPIRY_FIELD, mint)?;
    let now = Clock::get().unwrap().unix_timestamp;

    if expiry_ts.is_empty() {
        return err!(BurgerError::InvalidExpiryTS);
    }

    if now > expiry_ts.parse::<i64>().unwrap_or_default() {
        return err!(BurgerError::InvalidExpiryTS);
    }

    Ok(())
}

// pub fn check_valid_group(
//     collection_group: Account<'_, TokenGroupMember>,
//     game_group: Account<'_, GameConfig>
// ) -> Result<()> {
//     if collection_group.group == game_group.token_group {
//         return Ok(());
//     } else {
//         return err!(BurgerError::CollectionInvalid);
//     }
// }

// pub fn check_valid_mint(
//     mint: Pubkey,
//     group_member_mint: Pubkey,
// ) -> Result<()> {
//     if group_member_mint == mint {
//         return Ok(());
//     } else {
//         return err!(BurgerError::IncorrectMint);
//     }
// }
