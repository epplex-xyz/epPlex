use crate::*;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateFieldData {
    /// Field to update in the metadata
    pub field: AnchorField,
    /// Value to write for the field
    pub value: String,
}

// Need to do this so the enum shows up in the IDL
#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum AnchorField {
    /// The name field, corresponding to `TokenMetadata.name`
    Name,
    /// The symbol field, corresponding to `TokenMetadata.symbol`
    Symbol,
    /// The uri field, corresponding to `TokenMetadata.uri`
    Uri,
    /// A user field, whose key is given by the associated string
    Key(String),
}

// Convert AnchorField to Field from spl_token_metadata_interface
impl AnchorField {
    pub fn to_field(&self) -> Field {
        match self {
            AnchorField::Name => Field::Name,
            AnchorField::Symbol => Field::Symbol,
            AnchorField::Uri => Field::Uri,
            AnchorField::Key(s) => Field::Key(s.clone()),
        }
    }
}

