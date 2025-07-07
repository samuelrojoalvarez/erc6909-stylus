#![no_std]
extern crate alloc;

use alloc::{vec, vec::Vec};

use alloy_primitives::{Address, U256};
use stylus_sdk::{
    prelude::{storage, HostAccess, StorageType},
    storage::{StorageMap, StorageUint},
};

use super::super::{
    erc6909::Erc6909,
    error::Error,
    traits::{IErc6909, IErc6909Burnable, IErc6909Mintable, IErc6909Supply},
};

/// ERC-6909 + per-ID total-supply extension.
#[storage]
pub struct Erc6909Supply {
    /// The core ERC-6909 logic
    pub base: Erc6909,
    /// Tracks minted minus burned for each token ID
    total_supplies: StorageMap<U256, StorageUint<256, 4>>,
}

// --------------------------------------------------------------------------
// IErc6909Supply: just read out our `total_supplies` map
// --------------------------------------------------------------------------
impl IErc6909Supply for Erc6909Supply {
    fn total_supply(&self, id: U256) -> U256 {
        self.total_supplies.get(id)
    }
}

// --------------------------------------------------------------------------
// IErc6909: forward every core method to `base`
// --------------------------------------------------------------------------
impl IErc6909 for Erc6909Supply {
    fn balance_of(&self, owner: Address, id: U256) -> U256 {
        self.base.balance_of(owner, id)
    }

    fn allowance(&self, owner: Address, spender: Address, id: U256) -> U256 {
        self.base.allowance(owner, spender, id)
    }

    fn is_operator(&self, owner: Address, operator: Address) -> bool {
        self.base.is_operator(owner, operator)
    }

    fn approve(
        &mut self,
        caller: Address,
        spender: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, Error> {
        self.base.approve(caller, spender, id, amount)
    }

    fn set_operator(
        &mut self,
        caller: Address,
        operator: Address,
        approved: bool,
    ) -> Result<bool, Error> {
        self.base.set_operator(caller, operator, approved)
    }

    fn transfer(
        &mut self,
        caller: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, Error> {
        self.base.transfer(caller, to, id, amount)
    }

    fn transfer_from(
        &mut self,
        caller: Address,
        from: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, Error> {
        self.base.transfer_from(caller, from, to, id, amount)
    }
}

// --------------------------------------------------------------------------
// IErc6909Mintable: bump our total_supplies then mint balances
// --------------------------------------------------------------------------
impl IErc6909Mintable for Erc6909Supply {
    fn mint(
        &mut self,
        caller: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<(), Error> {
        // 1) update total_supplies[id] += amount
        let old = self.total_supplies.get(id);
        let new = old.checked_add(amount).ok_or(Error::ArithmeticOverflow)?;
        self.total_supplies.insert(id, new);
        // 2) mint the balance
        self.base.mint(caller, to, id, amount)
    }
}

// --------------------------------------------------------------------------
// IErc6909Burnable: burn balances then decrement total_supplies
// --------------------------------------------------------------------------
impl IErc6909Burnable for Erc6909Supply {
    fn burn(
        &mut self,
        caller: Address,
        from: Address,
        id: U256,
        amount: U256,
    ) -> Result<(), Error> {
        // 1) burn the balance
        self.base.burn(caller, from, id, amount)?;
        // 2) decrement total_supplies[id]
        let old = self.total_supplies.get(id);
        // you may want to guard underflow here; core already checks balances
        let new = old.checked_sub(amount).unwrap_or_default();
        self.total_supplies.insert(id, new);
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, U256};
    use stylus_sdk::testing::TestVM;

    /// Instantiate the supply extension out of storage so we can call it like a normal Rust struct.
    fn fresh_supply() -> Erc6909Supply {
        let vm = TestVM::default();
        Erc6909Supply::from(&vm)
    }

    #[test]
    fn mint_increases_total_supply() {
        let mut ext = fresh_supply();
        let caller = Address::new([0xAA; 20]);
        let id     = U256::from(42u64);
        let amount = U256::from(7u64);

        ext.mint(caller, caller, id, amount).unwrap();
        assert_eq!(ext.total_supply(id), amount);
    }

    #[test]
    fn burn_decreases_total_supply() {
        let mut ext = fresh_supply();
        let caller = Address::new([0xAA; 20]);
        let id     = U256::from(100u64);
        ext.mint(caller, caller, id, U256::from(10u64)).unwrap();

        ext.burn(caller, caller, id, U256::from(3u64)).unwrap();
        assert_eq!(ext.total_supply(id), U256::from(7u64));
    }

    #[test]
    fn mint_reverts_for_zero_caller_or_receiver() {
        let mut ext = fresh_supply();
        let zero = Address::new([0u8; 20]);
        let id   = U256::ONE;

        assert_eq!(
            ext.mint(zero, zero, id, U256::ONE),
            Err(Error::InvalidApprover)
        );
    }

    #[test]
    fn burn_reverts_when_underflow() {
        let mut ext = fresh_supply();
        let caller = Address::new([0xAA; 20]);
        let id     = U256::from(5u64);

        // burning without minting first
        assert_eq!(
            ext.burn(caller, caller, id, U256::ONE),
            Err(Error::InsufficientBalance)
        );
    }
}

// ——————————————————————————————————————————————————————————————————————————
// motsu-driven Unit tests
// ——————————————————————————————————————————————————————————————————————————
#[cfg(feature = "motsu")]
#[cfg(test)]
mod motsu_tests {
    use super::*;
    use alloy_primitives::U256;
    use stylus_sdk::{prelude::*, testing::TestVM};
    use motsu::prelude::*;

    // let Motsu know how to snapshot & rollback storage
    #[cfg(not(feature = "motsu"))]
    unsafe impl stylus_sdk::testing::TopLevelStorage for Erc6909MetadataUri {}

    fn fresh_contract() -> Contract<Erc6909MetadataUri> {
        let vm = TestVM::default();
        Contract::<Erc6909MetadataUri>::new(&vm)
    }

    #[motsu::test]
    fn set_and_read_back(mut c: Contract<Erc6909MetadataUri>, samuel: Address) {
        let id  = U256::from(42u64);
        let uri = b"motsu://token/42".to_vec();

        // samuel (non-zero) sets the URI
        c.sender(samuel)
        .set_token_uri(samuel, id, uri.clone())
        .motsu_unwrap();

        // And reading via the public getter returns the same bytes
        let got = c.token_uri(id);
        assert_eq!(got, uri);
    }

    #[motsu::test]
    fn zero_caller_fails(mut c: Contract<Erc6909MetadataUri>) {
        let zero = Address::new([0;20]);
        c.sender(zero)
        .set_token_uri(zero, U256::from(1u64), b"bad".to_vec())
        .motsu_unwrap_err();
    }
}
