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

// #![no_std]
// extern crate alloc;

// use alloc::vec::Vec;
// use alloy_primitives::{Address, U256};

// use super::error::Error;

// /// ERC-6909 core interface: single-ID multi-token standard.
// pub trait IErc6909 {
//     /// Returns the `owner`’s balance of token `id`.
//     fn balance_of(&self, owner: Address, id: U256) -> U256;

//     /// Returns how much `spender` is still allowed to transfer of `owner`’s
// `id`.     fn allowance(&self, owner: Address, spender: Address, id: U256) ->
// U256;

//     /// Returns true if `operator` is approved to manage *all* of `owner`’s
// tokens.     fn is_operator(&self, owner: Address, operator: Address) -> bool;

//     /// Grant `spender` permission to transfer up to `amount` of token `id`
// from `caller`.     ///
//     /// # Errors
//     /// - [`Error::InvalidApprover`] if `caller` is the zero address.
//     /// - [`Error::InvalidSpender`]  if `spender` is the zero address.
//     fn approve(
//         &mut self,
//         caller: Address,
//         spender: Address,
//         id: U256,
//         amount: U256,
//     ) -> Result<bool, Error>;

//     /// Set or unset `operator` as a global operator for `caller`.
//     ///
//     /// # Errors
//     /// - [`Error::InvalidApprover`] if `caller` is the zero address.
//     /// - [`Error::InvalidSpender`]  if `operator` is the zero address.
//     fn set_operator(
//         &mut self,
//         caller: Address,
//         operator: Address,
//         approved: bool,
//     ) -> Result<bool, Error>;

//     /// Transfer `amount` of `id` from `caller` to `to`.
//     ///
//     /// # Errors
//     /// - [`Error::InvalidSender`]   if `caller` is the zero address.
//     /// - [`Error::InvalidReceiver`] if `to` is the zero address.
//     /// - [`Error::InsufficientBalance`]
//     /// - [`Error::ArithmeticOverflow`]
//     fn transfer(
//         &mut self,
//         caller: Address,
//         to: Address,
//         id: U256,
//         amount: U256,
//     ) -> Result<bool, Error>;

//     /// Transfer `amount` of `id` from `from` to `to`, spending allowance or
// operator rights.     ///
//     /// # Errors
//     /// - [`Error::InvalidSender`]         if `from` is the zero address.
//     /// - [`Error::InvalidReceiver`]       if `to` is the zero address.
//     /// - [`Error::InsufficientAllowance`]
//     /// - [`Error::InsufficientBalance`]
//     /// - [`Error::ArithmeticOverflow`]
//     fn transfer_from(
//         &mut self,
//         caller: Address,
//         from: Address,
//         to: Address,
//         id: U256,
//         amount: U256,
//     ) -> Result<bool, Error>;
// }

// /// Thin wrapper over the core `_mint` hook.
// pub trait IErc6909Mintable {
//     /// Mint `amount` of `id` into `to`.
//     ///
//     /// # Errors
//     /// - [`Error::InvalidReceiver`]
//     /// - [`Error::ArithmeticOverflow`]
//     fn mint(
//         &mut self,
//         caller: Address,
//         to: Address,
//         id: U256,
//         amount: U256,
//     ) -> Result<(), Error>;
// }

// /// Thin wrapper over the core `_burn` hook.
// pub trait IErc6909Burnable {
//     /// Burn `amount` of `id` from `from`.
//     ///
//     /// # Errors
//     /// - [`Error::InvalidSender`]
//     /// - [`Error::InsufficientBalance`]
//     fn burn(
//         &mut self,
//         caller: Address,
//         from: Address,
//         id: U256,
//         amount: U256,
//     ) -> Result<(), Error>;
// }

// /// Supply‐tracking extension: total minted minus burned per ID.
// pub trait IErc6909Supply {
//     /// Returns total supply of token `id`.
//     fn total_supply(&self, id: U256) -> U256;
// }

// /// Simple metadata‐URI extension: store arbitrary bytes per ID.
// pub trait IErc6909MetadataUri {
//     /// Returns the URI bytes for `id`.
//     fn token_uri(&self, id: U256) -> Vec<u8>;

//     /// Sets the URI bytes for `id`.
//     ///
//     /// # Errors
//     /// - [`Error::InvalidApprover`] if `caller` is the zero address.
//     fn set_token_uri(
//         &mut self,
//         caller: Address,
//         id: U256,
//         uri: Vec<u8>,
//     ) -> Result<bool, Error>;
// }

// /// Enumerable extension: list and index all known IDs.
// pub trait IErc6909Enumerable {
//     /// Returns a `Vec` of *all* token IDs that have ever existed.
//     fn total_ids(&self) -> Vec<U256>;

//     /// Returns the token ID at position `index`.
//     ///
//     /// # Panics
//     /// If `index >= total_ids().len()`.
//     fn id_by_index(&self, index: U256) -> U256;
// }
