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
