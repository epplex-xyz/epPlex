use crate::*;
use epplex_shared::{BITS_8, DISCRIMINATOR_LENGTH};

#[constant]
pub const SEED_BURGER_METADATA: &[u8] = b"burgermetadata";

pub const ENCRYPTED_LENTH: usize = 172;

/// Reserve 200 bytes
pub const GAME_STATE_PLACEHOLDER: &str =
    "99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999";
pub const GAME_STATE: &str = "gameState";

/// Length of 20 characters unixtimestamp
pub const VOTING_TIMESTAMP_PLACEHOLDER: &str = "99999999999999999999";
pub const VOTING_TIMESTAMP: &str = "votingTimestamp";

// false, true
pub const IMMUNITY: &str = "immunity";
pub const NEW_IMMUNITY: &str = "@immunity";
pub const IMMUNITY_PLACEHOLDER: &str = "false";

// Fields
pub const EXPIRY_FIELD: &str = "expirationDate"; // should just add onto this

/// Length of 60 characters for community name
// pub const COMMUNITY_PLACEHOLDER: &str =
//     "999999999999999999999999999999999999999999999999999999999999";
// pub const COMMUNITY: &str = "DAO";

pub fn generate_metadata(expiry_data: String) -> Vec<[String; 2]> {
    vec![
        [EXPIRY_FIELD.to_string(), expiry_data],
        [GAME_STATE.to_string(), GAME_STATE_PLACEHOLDER.to_string()],
        [
            VOTING_TIMESTAMP.to_string(),
            VOTING_TIMESTAMP_PLACEHOLDER.to_string(),
        ],
        [IMMUNITY.to_string(), IMMUNITY_PLACEHOLDER.to_string()],
    ]
}

pub fn generate_metadata2(expiry_data: String) -> Vec<wen_new_standard::types::AddMetadataArgs> {
    vec![
        AddMetadataArgs {
            field: EXPIRY_FIELD.to_string(),
            value: expiry_data,
        },
        AddMetadataArgs {
            field: GAME_STATE.to_string(),
            value: GAME_STATE_PLACEHOLDER.to_string(),
        },
        AddMetadataArgs {
            field: VOTING_TIMESTAMP.to_string(),
            value: VOTING_TIMESTAMP_PLACEHOLDER.to_string(),
        },
        AddMetadataArgs {
            field: IMMUNITY.to_string(),
            value: IMMUNITY_PLACEHOLDER.to_string(),
        },
    ]
}

#[account]
#[derive(Default, Debug)]
pub struct BurgerMetadata {
    /// The bump, used for PDA validation.
    pub bump: u8,
    // During game evaluation phase, we need to make sure that all of these are flipped
    // Although, what if they burn themselves?
    // pub is_processed: u8,
}

impl BurgerMetadata {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        // + BITS_8
        + BITS_8;

    pub fn new(bump: u8) -> Self {
        Self {
            bump,
            // is_processed: 0
        }
    }
}
