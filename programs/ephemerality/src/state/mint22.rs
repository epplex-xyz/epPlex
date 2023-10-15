use crate::*;

pub const METADATA_OFFSET: usize = 374;

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct Metadata {
    pub name: String, // 374
    pub symbol: String,
    pub uri: String,

    pub dunno7: [u8; 4],
    pub destroy_timestamp_field: String,
    pub destroy_timestamp_value: String,
}

// TODO this is super hardcoded
#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Mint22 {
    /// Optional authority used to mint new tokens. The mint authority may only be provided during
    /// mint creation. If no mint authority is present then the mint has a fixed supply and no
    /// further tokens may be minted.
    pub mint_authority: Option<Pubkey>,
    /// Total supply of tokens.
    pub supply: u64,
    /// Number of base 10 digits to the right of the decimal place.
    pub decimals: u8,
    /// Is `true` if this structure has been initialized
    pub is_initialized: bool,
    /// Optional authority to freeze token accounts.
    pub freeze_authority: Option<Pubkey>,

    pub padding: [u8; 83], // 165
    pub dunno1: [u8; 5], // 170
    pub close_authority: Pubkey, // 202

    pub dunno2: [u8; 4], // 206
    pub permanent_delegate: Pubkey, // 238

    pub dunno3: [u8; 4], // 242

    pub dunno4: Pubkey, // 274

    pub dunno5: Pubkey, // 306

    pub dunno6: [u8; 4], // 310

    pub metadata_pointer_authority: Pubkey, // 342
    pub metadata_address: Pubkey, // 374

    pub name: String, // 374
    pub symbol: String,
    pub uri: String,

    pub dunno7: [u8; 4],
    pub destroy_timestamp_field: String,
    pub destroy_timestamp_value: String,
}

impl Default for Mint22 {
    fn default() -> Self {
        Mint22 {
            mint_authority: Default::default(),
            supply: Default::default(),
            decimals: Default::default(),
            is_initialized: Default::default(),
            freeze_authority: Default::default(),
            // initialize other fields with their default values
            padding: [0; 83],
            dunno1: Default::default(),
            close_authority: Default::default(),
            dunno2: Default::default(),
            permanent_delegate: Default::default(),
            dunno3: Default::default(),
            dunno4: Default::default(),
            dunno5: Default::default(),
            dunno6: Default::default(),
            metadata_pointer_authority: Default::default(),
            metadata_address: Default::default(),
            name: Default::default(),
            symbol: Default::default(),
            uri: Default::default(),
            dunno7: Default::default(),
            destroy_timestamp_field: Default::default(),
            destroy_timestamp_value: Default::default(),
        }
    }
}