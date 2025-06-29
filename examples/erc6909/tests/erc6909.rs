use motsu::prelude::*;
use alloy_primitives::{Address, U256};
use openzeppelin_stylus::token::erc6909::{Erc6909Supply, Error};

#[motsu::test]
fn mint_and_transfer(env: TestEnv) {
    let mut tok = Erc6909Supply::default();
    let alice = Address::repeat_byte(0xAA);
    let bob   = Address::repeat_byte(0xBB);
    let id = U256::from(1);

    // mint
    tok.mint(alice, id, 100u8.into()).unwrap();
    assert_eq!(tok.base.balance_of(alice, id), 100u8.into());
    assert_eq!(tok.total_supply(id),            100u8.into());

    // transfer 40 to Bob
    tok.base
        .transfer_from(alice, alice, bob, id, 40u8.into())
        .unwrap();
    assert_eq!(tok.base.balance_of(alice, id), 60u8.into());
    assert_eq!(tok.base.balance_of(bob,   id), 40u8.into());
    assert_eq!(tok.total_supply(id),            100u8.into());
}
