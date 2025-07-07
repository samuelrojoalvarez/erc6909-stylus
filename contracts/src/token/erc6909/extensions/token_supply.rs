#![no_std]
extern crate alloc;

use alloc::{vec, vec::Vec};

use alloy_primitives::{Address, U256};
use stylus_sdk::{
    prelude::{storage, HostAccess, StorageType},
    storage::{StorageMap, StorageUint},
};


// === change these imports to point at the core ERC-6909 and its traits ===
use super::super::{
    erc6909::Erc6909,
    error::Error,
    traits::{IErc6909, IErc6909Mintable, IErc6909Burnable, IErc6909Supply},
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



// ——————————————————————————————————————————————————————————————————————————
// motsu-driven integration tests
// Run : cargo test -p openzeppelin-stylus --features stylus-test
// ——————————————————————————————————————————————————————————————————————————
#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::{Address, U256};
    use stylus_sdk::testing::TestVM;
    use motsu::prelude::*;

    // Tell Motsu how to snapshot/rollback this pure-storage type
    #[cfg(not(feature = "erc6909"))]
    unsafe impl stylus_sdk::testing::TopLevelStorage for Erc6909Supply {}

    /// Helper to get a fresh `Contract<Erc6909Supply>` and a random account
    fn fresh() -> Contract<Erc6909Supply> {
        Contract::<Erc6909Supply>::new(&TestVM::default())
    }

    #[motsu::test]
    fn initial_total_is_zero(mut c: Contract<Erc6909Supply>, owner: Address) {
        let id = U256::from(1u64);
        // Nothing minted yet → total_supply must be zero
        assert_eq!(c.total_supply(id), U256::ZERO);
    }

    #[motsu::test]
    fn mint_increases_total(mut c: Contract<Erc6909Supply>, owner: Address) {
        let id     = U256::from(7u64);
        let amount = U256::from(42u64);

        // Mint `amount` to `owner`
        c.mint(owner, owner, id, amount).motsu_unwrap();

        // Now the total supply for `id` should be exactly `amount`
        assert_eq!(c.total_supply(id), amount);
    }

    #[motsu::test]
    fn burn_decreases_total(mut c: Contract<Erc6909Supply>, owner: Address) {
        let id         = U256::from(10u64);
        let minted_amt = U256::from(50u64);
        let burn_amt   = U256::from(15u64);

        // Mint then burn
        c.mint(owner, owner, id, minted_amt).motsu_unwrap();
        c.burn(owner, owner, id, burn_amt).motsu_unwrap();

        // Remaining supply = minted_amt - burn_amt
        assert_eq!(c.total_supply(id), minted_amt - burn_amt);
    }

    #[motsu::test]
    fn burn_without_mint_reverts(mut c: Contract<Erc6909Supply>, owner: Address) {
        let id = U256::from(99u64);

        // Trying to burn an ID that was never minted should revert
        c.burn(owner, owner, id, U256::ONE)
            .motsu_unwrap_err();
    }
}

