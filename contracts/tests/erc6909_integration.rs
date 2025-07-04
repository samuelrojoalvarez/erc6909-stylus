use alloy_primitives::{Address, U256};
use motsu::prelude::*;
use openzeppelin_stylus::token::erc6909::{traits::*, Erc6909};
use stylus_sdk::testing::TestVM;

#[motsu::test]
fn full_mint_transfer_burn_flow() {
    let vm = TestVM::default();
    let mut token = Erc6909::from(&vm);
    let owner = Address::new([0xAA; 20]);
    let recipient = Address::new([0xBB; 20]);
    let id = U256::from(7u64);

    // mint
    token.mint(owner, owner, id, U256::from(100u64)).unwrap();
    assert_eq!(token.balance_of(owner, id), U256::from(100u64));

    // approve + transfer_from
    token.approve(owner, recipient, id, U256::from(30u64)).unwrap();
    assert!(token
        .transfer_from(recipient, owner, recipient, id, U256::from(20u64))
        .unwrap());
    assert_eq!(token.balance_of(recipient, id), U256::from(20u64));
    assert_eq!(token.allowance(owner, recipient, id), U256::from(10u64));

    // burn
    token.burn(owner, owner, id, U256::from(10u64)).unwrap();
    assert_eq!(token.balance_of(owner, id), U256::from(70u64));
}
