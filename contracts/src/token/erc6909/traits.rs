#![no_std]
extern crate alloc;

use alloc::vec::Vec;

use alloy_primitives::{Address, U256};

use super::error::Error;

/// Core ERC-6909 interface: single-ID multi-token.
pub trait IErc6909 {
    fn balance_of(&self, owner: Address, id: U256) -> U256;
    fn allowance(&self, owner: Address, spender: Address, id: U256) -> U256;
    fn is_operator(&self, owner: Address, operator: Address) -> bool;

    fn approve(
        &mut self,
        caller: Address,
        spender: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, Error>;

    fn set_operator(
        &mut self,
        caller: Address,
        operator: Address,
        approved: bool,
    ) -> Result<bool, Error>;

    fn transfer(
        &mut self,
        caller: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, Error>;

    fn transfer_from(
        &mut self,
        caller: Address,
        from: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, Error>;
}

/// Mintable extension
pub trait IErc6909Mintable {
    fn mint(
        &mut self,
        caller: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<(), Error>;
}

/// Burnable extension
pub trait IErc6909Burnable {
    fn burn(
        &mut self,
        caller: Address,
        from: Address,
        id: U256,
        amount: U256,
    ) -> Result<(), Error>;
}

/// Supply-tracking extension
pub trait IErc6909Supply {
    fn total_supply(&self, id: U256) -> U256;
}

/// Metadata-URI extension
pub trait IErc6909MetadataUri {
    fn token_uri(&self, id: U256) -> Vec<u8>;
    fn set_token_uri(
        &mut self,
        caller: Address,
        id: U256,
        uri: Vec<u8>,
    ) -> Result<bool, Error>;
}

/// Enumerable extension trait for ERC-6909.
pub trait IErc6909Enumerable {
    /// Returns total unique IDs recorded.
    fn total_ids(&self) -> U256;
    /// Returns the ID at `index`, or zero if out of bounds.
    fn id_by_index(&self, index: U256) -> U256;
}

