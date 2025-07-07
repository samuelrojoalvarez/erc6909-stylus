#![no_std]

pub mod erc6909;
pub mod error;
pub mod extensions;
pub mod traits;


// ─── Core exports ───────────────────────────────────────────────────────────
pub use erc6909::Erc6909;
pub use error::Error;

// ─── Extension *types* ──────────────────────────────────────────────────────
pub use extensions::metadata_uri::Erc6909MetadataUri;
pub use extensions::token_supply::Erc6909Supply;

// ─── All of the ERC-6909 traits ──────────────────────────────────────────────
pub use traits::{
    IErc6909,
    IErc6909Mintable,
    IErc6909Burnable,
    IErc6909Supply,
    IErc6909MetadataUri,
};
