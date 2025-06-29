// use core::str::FromStr;
// use alloy_primitives::{Address, U256};
// use openzeppelin_stylus::token::erc6909::{
//     extensions::token_supply::Erc6909Supply,
//     traits::IErc6909Supply,
// };
// use stylus_sdk::testing::TestVM;
// use openzeppelin_stylus::token::erc6909::traits::IErc6909Mintable;


// fn main() {
//     // 1) spin up an in-memory VM
//     let vm = TestVM::default();

//     // 2) instantiate our extension-backed contract
//     let mut token = Erc6909Supply::from(&vm);

//     // 3) a test address
//     let alice = Address::new([0xAA; 20]);

//     // 4) prepare ID and amount
//     let id     = U256::from_str("42").unwrap();
//     let amount = U256::from_str("100").unwrap();

//     // 5) mint and check total supply
//     token.mint(alice, alice, id, amount).unwrap();
//     assert_eq!(token.total_supply(id), amount);

//     println!("Alice balance is {}", token.total_supply(id));
//     println!("✅ erc6909-supply example OK");
// }

// examples/erc6909-supply.rs

use core::str::FromStr;
use alloy_primitives::{Address, U256};
use openzeppelin_stylus::token::erc6909::{
    extensions::token_supply::Erc6909Supply,
    traits::IErc6909Supply,
};
use openzeppelin_stylus::token::erc6909::traits::IErc6909Mintable;
use stylus_sdk::testing::TestVM;

fn main() {
    // 1) spin up an in-memory VM
    let vm = TestVM::default();

    // 2) instantiate our extension-backed contract
    let mut token = Erc6909Supply::from(&vm);

    // 3) a test address (Alice)
    let alice = Address::new([0xAA; 20]);

    // 4) prepare ID and amount
    let id     = U256::from_str("42").unwrap();
    let amount = U256::from_str("100").unwrap();

    // 5) mint and check total supply
    //    — caller = alice, recipient = alice
    token.mint(alice, alice, id, amount).unwrap();
    assert_eq!(token.total_supply(id), amount);

    println!("Alice total supply for ID {} is {}", id, token.total_supply(id));
    println!("✅ erc6909-supply example OK");
}
