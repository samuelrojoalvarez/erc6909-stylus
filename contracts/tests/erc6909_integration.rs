use motsu::prelude::*;
use stylus_sdk::testing::TestVM;
use alloy_primitives::{Address, U256};
use openzeppelin_stylus::token::erc6909::Erc6909;
use openzeppelin_stylus::token::erc6909::traits::*;

#[motsu::test]
fn full_mint_transfer_burn_flow() {
    let vm = TestVM::default();
    let mut token = Erc6909::from(&vm);
    let alice = Address::new([0xAA;20]);
    let bob   = Address::new([0xBB;20]);
    let id    = U256::from(7u64);

    // mint
    token.mint(alice, alice, id, U256::from(100u64)).unwrap();
    assert_eq!(token.balance_of(alice, id), U256::from(100u64));

    // approve + transfer_from
    token.approve(alice, bob, id, U256::from(30u64)).unwrap();
    assert!(token.transfer_from(bob, alice, bob, id, U256::from(20u64)).unwrap());
    assert_eq!(token.balance_of(bob, id), U256::from(20u64));
    assert_eq!(token.allowance(alice, bob, id), U256::from(10u64));

    // burn
    token.burn(alice, alice, id, U256::from(10u64)).unwrap();
    assert_eq!(token.balance_of(alice, id), U256::from(70u64));
}


