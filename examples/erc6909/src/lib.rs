#![no_std]

use stylus_sdk::{entrypoint, storage};
use alloy_primitives::{Address, U256};
use openzeppelin_stylus::token::erc6909::{
    Erc6909Supply, IErc6909, IErc6909Supply, Error,
};

#[entrypoint]
#[storage]
pub struct MultiToken {
    token: Erc6909Supply,
}

impl IErc6909 for MultiToken {
    fn balance_of(&self, owner: Address, id: U256) -> U256 {
        self.token.base.balance_of(owner, id)
    }
    fn allowance(&self, owner: Address, spender: Address, id: U256) -> U256 {
        self.token.base.allowance(owner, spender, id)
    }
    fn is_operator(&self, owner: Address, operator: Address) -> bool {
        self.token.base.is_operator(owner, operator)
    }
    fn approve(
        &mut self,
        caller: Address,
        spender: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, Error> {
        self.token.base.approve(caller, spender, id, amount)
    }
    fn set_operator(
        &mut self,
        caller: Address,
        operator: Address,
        approved: bool,
    ) -> Result<bool, Error> {
        self.token.base.set_operator(caller, operator, approved)
    }
    fn transfer(
        &mut self,
        caller: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, Error> {
        self.token.base.transfer(caller, to, id, amount)
    }
    fn transfer_from(
        &mut self,
        caller: Address,
        from: Address,
        to: Address,
        id: U256,
        amount: U256,
    ) -> Result<bool, Error> {
        self.token.base.transfer_from(caller, from, to, id, amount)
    }
}

impl IErc6909Supply for MultiToken {
    fn total_supply(&self, id: U256) -> U256 {
        self.token.total_supply(id)
    }
}
