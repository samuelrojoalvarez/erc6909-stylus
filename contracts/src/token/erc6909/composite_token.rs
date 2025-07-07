#![no_std]
extern crate alloc;

use alloc::{vec, vec::Vec};
use alloy_primitives::{Address, U256};
use stylus_sdk::prelude::*; // entrypoint, storage, public, implements, HostAccess, StorageType

use crate::token::erc6909::error::Error;
use crate::token::erc6909::erc6909::Erc6909;
use crate::token::erc6909::extensions::{Erc6909Enumerable, Erc6909MetadataUri, Erc6909Supply};
use crate::token::erc6909::traits::{
    IErc6909,
    IErc6909Mintable,
    IErc6909Burnable,
    IErc6909Enumerable,
    IErc6909MetadataUri,
    IErc6909Supply,
};

/// A “one-stop” ERC-6909: core + enumerable + metadata + supply
#[cfg_attr(feature = "erc6909", entrypoint)]
#[storage]
pub struct MyToken {
    core:       Erc6909,
    enumerable: Erc6909Enumerable,
    metadata:   Erc6909MetadataUri,
    supply:     Erc6909Supply,
}

// for Motsu snapshot/rollback
unsafe impl stylus_sdk::testing::TopLevelStorage for MyToken {}

/// Implement **all** six interfaces in one router impl:
#[public]
#[implements(
    IErc6909<Error = Error>,
    IErc6909Mintable<Error = Error>,
    IErc6909Burnable<Error = Error>,
    IErc6909Enumerable,
    IErc6909MetadataUri,
    IErc6909Supply
)]
impl MyToken {
    // — Core IErc6909 —
    fn balance_of(&self, owner: Address, id: U256) -> U256 {
        self.core.balance_of(owner, id)
    }
    fn allowance(&self, owner: Address, spender: Address, id: U256) -> U256 {
        self.core.allowance(owner, spender, id)
    }
    fn is_operator(&self, owner: Address, operator: Address) -> bool {
        self.core.is_operator(owner, operator)
    }
    fn approve(
        &mut self,
        caller:  Address,
        spender: Address,
        id:      U256,
        amount:  U256,
    ) -> Result<bool, Error> {
        self.core.approve(caller, spender, id, amount)
    }
    fn set_operator(
        &mut self,
        caller:   Address,
        operator: Address,
        approved: bool,
    ) -> Result<bool, Error> {
        self.core.set_operator(caller, operator, approved)
    }
    fn transfer(
        &mut self,
        caller: Address,
        to:     Address,
        id:     U256,
        amount: U256,
    ) -> Result<bool, Error> {
        self.core.transfer(caller, to, id, amount)
    }
    fn transfer_from(
        &mut self,
        caller: Address,
        from:   Address,
        to:     Address,
        id:     U256,
        amount: U256,
    ) -> Result<bool, Error> {
        self.core.transfer_from(caller, from, to, id, amount)
    }

    // — Mintable + Enumerable —
    fn mint(
        &mut self,
        caller: Address,
        to:     Address,
        id:     U256,
        amount: U256,
    ) -> Result<(), Error> {
        self.core.mint(caller, to, id, amount)?;
        self.supply.mint(caller, to, id, amount)?;
        self.enumerable._record_id(id);
        Ok(())
    }

    // — Burnable + Supply —
    fn burn(
        &mut self,
        caller: Address,
        from:   Address,
        id:     U256,
        amount: U256,
    ) -> Result<(), Error> {
        self.core.burn(caller, from, id, amount)?;
        self.supply.burn(caller, from, id, amount)?;
        Ok(())
    }

    // — Enumerable-only —
    fn total_ids(&self) -> U256 {
        self.enumerable.total_ids()
    }
    fn id_by_index(&self, index: U256) -> U256 {
        self.enumerable.id_by_index(index)
    }

    // — Metadata-only —
    fn token_uri(&self, id: U256) -> Vec<u8> {
        self.metadata.token_uri(id)
    }
    fn set_token_uri(
        &mut self,
        caller: Address,
        id:     U256,
        uri:    Vec<u8>,
    ) -> Result<bool, Error> {
        self.metadata.set_token_uri(caller, id, uri)
    }

    // — Supply-only —
    fn total_supply(&self, id: U256) -> U256 {
        self.supply.total_supply(id)
    }
}



// ——————————————————————————————————————————————————————————————————————————
// motsu-driven Unit tests
// Run : cargo test -p openzeppelin-stylus --features stylus-test
// ——————————————————————————————————————————————————————————————————————————
#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, U256};
    use stylus_sdk::testing::TestVM;
    use motsu::prelude::*;

    fn fresh() -> Contract<MyToken> {
        let vm = TestVM::default();
        Contract::<MyToken>::new(&vm)
    }

    #[motsu::test]
    fn full_mint_transfer_burn_flow(
        mut c: Contract<MyToken>,
        samuel: Account,
        mother:   Account,
    ) {
        let id = U256::from(7u64);

        // mint
        c.mint(samuel.address(), samuel.address(), id, U256::from(100))
            .motsu_unwrap();
        assert_eq!(c.balance_of(samuel.address(), id), U256::from(100));
        assert_eq!(c.total_supply(id), U256::from(100));

        // transfer + enumerate
        c.approve(samuel.address(), mother.address(), id, U256::from(50))
            .motsu_unwrap();
        c.transfer_from(
            samuel.address(),
            samuel.address(),
            mother.address(),
            id,
            U256::from(30),
        )
        .motsu_unwrap();
        assert_eq!(c.balance_of(mother.address(), id), U256::from(30));
        assert_eq!(c.total_ids(), U256::from(1));
        assert_eq!(c.id_by_index(U256::ZERO), id);

        // burn
        c.burn(samuel.address(), samuel.address(), id, U256::from(10))
            .motsu_unwrap();
        assert_eq!(c.total_supply(id), U256::from(90));
    }

    #[motsu::test]
    fn metadata_round_trip(mut c: Contract<MyToken>, samuel: Account) {
        let id = U256::from(42);
        let uri = b"ipfs://foo".to_vec();
        c.set_token_uri(samuel.address(), id, uri.clone())
            .motsu_unwrap();
        assert_eq!(c.token_uri(id), uri);
    }

    #[motsu::test]
    fn enumeration_handles_duplicates(mut c: Contract<MyToken>, samuel: Account) {
        let id = U256::from(99);
        c.mint(samuel.address(), samuel.address(), id, U256::ONE)
            .motsu_unwrap();
        c.mint(samuel.address(), samuel.address(), id, U256::ONE)
            .motsu_unwrap();
        assert_eq!(c.total_ids(), U256::ONE);
    }

    #[motsu::test]
    fn out_of_bounds_index_returns_zero(mut c: Contract<MyToken>) {
        assert_eq!(c.id_by_index(U256::from(5)), U256::ZERO);
    }

    #[motsu::test]
    fn invalid_zero_caller_reverts(mut c: Contract<MyToken>) {
        let id   = U256::ONE;
        let zero = Address::new([0; 20]);
        c.set_token_uri(zero, id, b"x".to_vec()).motsu_unwrap_err();
        c.mint(zero, zero, id, U256::ONE).motsu_unwrap_err();
        c.burn(zero, zero, id, U256::ONE).motsu_unwrap_err();
    }

    #[motsu::test]
    fn test_token_transfers(token: Contract<MyToken>, samuel: Account, mother: Account) {
        let id     = U256::from(1);
        let amount = U256::from(100);

        // mint 100 to samuel
        token
            .sender(samuel)
            .mint(samuel.address(), samuel.address(), id, amount)
            .motsu_unwrap();

        // samuel’s balance is 100, mother’s is 0
        assert_eq!(
            token.sender(samuel).balance_of(samuel.address(), id),
            amount
        );
        assert_eq!(
            token.sender(samuel).balance_of(mother.address(),   id),
            U256::ZERO
        );

        // samuel approves mother for 30
        token
            .sender(samuel)
            .approve(samuel.address(), mother.address(), id, U256::from(30))
            .motsu_unwrap();

        // mother pulls 30 out of samuel’s balance
        token
            .sender(mother)
            .transfer_from(samuel.address(), samuel.address(), mother.address(), id, U256::from(30))
            .motsu_unwrap();

        // balances updated
        assert_eq!(
            token.sender(samuel).balance_of(samuel.address(), id),
            amount - U256::from(30)
        );
        assert_eq!(
            token.sender(mother).balance_of(mother.address(), id),
            U256::from(30)
        );

        // enumeration picked up that token‐ID
        assert_eq!(token.total_ids(), U256::from(1));
        assert_eq!(token.id_by_index(U256::ZERO), id);

        // if samuel tries to send more than she has: revert
        let too_much = amount * U256::from(2);
        let err = token
            .sender(samuel)
            .transfer(mother.address(), id, too_much)
            .motsu_unwrap_err();
         assert!(matches!(err, erc6909::Error::InsufficientBalance(_)));
    }
}
