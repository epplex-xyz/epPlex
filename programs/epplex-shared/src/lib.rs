use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod program_wrappers;
pub use program_wrappers::*;


declare_id!("Cxtfv4jRc6H7Yb8gVpdt7tTFte8RWEa8RgMk61qhRdiP");

#[program]
pub mod epplex_shared {}