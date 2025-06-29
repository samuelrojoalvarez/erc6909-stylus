// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use stylus_sdk::testing::TestVM;
// use openzeppelin_stylus::token::erc6909::{
//     erc6909::Erc6909,
//     traits::{IErc6909, IErc6909Mintable, IErc6909Burnable},
// };
// use alloy_primitives::{Address, U256};

// fn setup() -> (Erc6909, Address, Address, U256) {
//     let vm    = TestVM::default();
//     let mut token = Erc6909::from(&vm);
//     let alice = Address::new([0xAA; 20]);
//     let bob   = Address::new([0xBB; 20]);
//     let id    = U256::from(1u64);
//     (token, alice, bob, id)
// }

// pub fn bench_approve(c: &mut Criterion) {
//     let (mut t, alice, bob, id) = setup();
//     c.bench_function("erc6909 approve", |b| {
//         b.iter(|| {
//             t.approve(
//                 black_box(alice),
//                 black_box(bob),
//                 black_box(id),
//                 black_box(U256::from(42u64)),
//             ).unwrap();
//         })
//     });
// }

// pub fn bench_transfer(c: &mut Criterion) {
//     let (mut t, alice, bob, id) = setup();
//     // give alice some balance
//     t.mint(alice, alice, id, U256::from(100u64)).unwrap();
//     c.bench_function("erc6909 transfer", |b| {
//         b.iter(|| {
//             t.transfer(
//                 black_box(alice),
//                 black_box(bob),
//                 black_box(id),
//                 black_box(U256::from(1u64)),
//             ).unwrap();
//         })
//     });
// }

// pub fn bench_transfer_from(c: &mut Criterion) {
//     let (mut t, alice, bob, id) = setup();
//     t.mint(alice, alice, id, U256::from(100u64)).unwrap();
//     t.approve(alice, bob, id, U256::from(50u64)).unwrap();
//     c.bench_function("erc6909 transfer_from", |b| {
//         b.iter(|| {
//             t.transfer_from(
//                 black_box(bob),
//                 black_box(alice),
//                 black_box(bob),
//                 black_box(id),
//                 black_box(U256::from(1u64)),
//             ).unwrap();
//         })
//     });
// }

// pub fn bench_burn(c: &mut Criterion) {
//     let (mut t, alice, _bob, id) = setup();
//     t.mint(alice, alice, id, U256::from(100u64)).unwrap();
//     c.bench_function("erc6909 burn", |b| {
//         b.iter(|| {
//             t.burn(
//                 black_box(alice),
//                 black_box(alice),
//                 black_box(id),
//                 black_box(U256::from(1u64)),
//             ).unwrap();
//         })
//     });
// }

// criterion_group!(core_benches, bench_approve, bench_transfer, bench_transfer_from, bench_burn);
// criterion_main!(core_benches);

// benches/erc6909_core.rs
// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use stylus_sdk::testing::TestVM;
// use alloy_primitives::{Address, U256};

// use openzeppelin_stylus::token::erc6909::Erc6909Core;               // your core type
// use openzeppelin_stylus::token::erc6909::traits::IErc6909Core;    // approve/transfer/burn

// fn setup() -> (Erc6909Core, Address, Address, U256) {
//     let vm = TestVM::default();
//     let token = Erc6909Core::from(&vm);
//     let alice = Address::new([0xAA; 20]);
//     let bob   = Address::new([0xBB; 20]);
//     let id    = U256::from(1);
//     // give alice some balance:
//     token.mint(alice, id, U256::from(100u64)).unwrap();
//     (token, alice, bob, id)
// }

// fn bench_approve(c: &mut Criterion) {
//     let (mut token, alice, bob, id) = setup();
//     c.bench_function("erc6909_core approve", |b| {
//         b.iter(|| {
//             token
//                 .approve(
//                     black_box(alice),
//                     black_box(bob),
//                     black_box(id),
//                     black_box(U256::from(50u64)),
//                 )
//                 .unwrap();
//         })
//     });
// }

// fn bench_transfer(c: &mut Criterion) {
//     let (mut token, alice, bob, id) = setup();
//     c.bench_function("erc6909_core transfer", |b| {
//         b.iter(|| {
//             token
//                 .transfer(
//                     black_box(alice),
//                     black_box(bob),
//                     black_box(id),
//                     black_box(U256::from(10u64)),
//                 )
//                 .unwrap();
//         })
//     });
// }

// fn bench_transfer_from(c: &mut Criterion) {
//     let (mut token, alice, bob, id) = setup();
//     // pre-approve bob to move on behalf of alice
//     token
//         .approve(alice, bob, id, U256::from(20u64))
//         .unwrap();
//     c.bench_function("erc6909_core transfer_from", |b| {
//         b.iter(|| {
//             token
//                 .transfer_from(
//                     black_box(bob),
//                     black_box(alice),
//                     black_box(bob),
//                     black_box(id),
//                     black_box(U256::from(5u64)),
//                 )
//                 .unwrap();
//         })
//     });
// }

// fn bench_burn(c: &mut Criterion) {
//     let (mut token, alice, _bob, id) = setup();
//     c.bench_function("erc6909_core burn", |b| {
//         b.iter(|| {
//             token
//                 .burn(
//                     black_box(alice),
//                     black_box(id),
//                     black_box(U256::from(5u64)),
//                 )
//                 .unwrap();
//         })
//     });
// }

// // once you’ve wired-up metadata:
// fn bench_set_token_uri(c: &mut Criterion) {
//     let (mut token, _alice, _bob, id) = setup();
//     let uri = black_box(b"http://example.com/".repeat(10));  // 110 bytes
//     c.bench_function("erc6909_core set_token_uri(100B)", |b| {
//         b.iter(|| {
//             token
//                 .set_token_uri(black_box(id), uri.clone())
//                 .unwrap();
//         })
//     });
// }

// fn bench_token_uri(c: &mut Criterion) {
//     let (mut token, _alice, _bob, id) = setup();
//     let uri: Vec<u8> = vec![b'x'; 100];
//     token.set_token_uri(id, uri.clone()).unwrap();
//     c.bench_function("erc6909_core token_uri(100B)", |b| {
//         b.iter(|| {
//             black_box(token.token_uri(black_box(id)));
//         })
//     });
// }

// criterion_group!(
//     core_benches,
//     bench_approve,
//     bench_transfer,
//     bench_transfer_from,
//     bench_burn,
//     bench_set_token_uri,
//     bench_token_uri,
// );
// criterion_main!(core_benches);




// benches/erc6909_core.rs

// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use stylus_sdk::testing::TestVM;
// use alloy_primitives::{Address, U256};

// // Core ERC-6909 implementation and its traits
// use openzeppelin_stylus::token::erc6909::Erc6909;
// use openzeppelin_stylus::token::erc6909::traits::{
//     IErc6909,
//     IErc6909Mintable,
//     IErc6909Burnable,
// };

// /// Sets up a fresh token instance and mints initial balance for `alice`.
// fn setup() -> (Erc6909, Address, Address, U256) {
//     let vm = TestVM::default();
//     let mut token = Erc6909::from(&vm);
//     let alice = Address::new([0xAA; 20]);
//     let bob = Address::new([0xBB; 20]);
//     let id = U256::from(1u64);
//     // Provide Alice with some initial supply to allow transfers/burns
//     token.mint(alice, alice, id, U256::from(100u64)).unwrap();
//     (token, alice, bob, id)
// }

// fn bench_approve(c: &mut Criterion) {
//     let (mut token, alice, bob, id) = setup();
//     c.bench_function("erc6909_core approve", |b| {
//         b.iter(|| {
//             token
//                 .approve(
//                     black_box(alice),
//                     black_box(bob),
//                     black_box(id),
//                     black_box(U256::from(50u64)),
//                 )
//                 .unwrap();
//         });
//     });
// }

// fn bench_transfer(c: &mut Criterion) {
//     let (mut token, alice, bob, id) = setup();
//     c.bench_function("erc6909_core transfer", |b| {
//         b.iter(|| {
//             token
//                 .transfer(
//                     black_box(alice),
//                     black_box(bob),
//                     black_box(id),
//                     black_box(U256::from(10u64)),
//                 )
//                 .unwrap();
//         });
//     });
// }

// fn bench_transfer_from(c: &mut Criterion) {
//     let (mut token, alice, bob, id) = setup();
//     // Pre-approve Bob to move Alice's tokens
//     token
//         .approve(alice, bob, id, U256::from(20u64))
//         .unwrap();
//     c.bench_function("erc6909_core transfer_from", |b| {
//         b.iter(|| {
//             token
//                 .transfer_from(
//                     black_box(bob),
//                     black_box(alice),
//                     black_box(bob),
//                     black_box(id),
//                     black_box(U256::from(5u64)),
//                 )
//                 .unwrap();
//         });
//     });
// }

// fn bench_burn(c: &mut Criterion) {
//     let (mut token, alice, _, id) = setup();
//     c.bench_function("erc6909_core burn", |b| {
//         b.iter(|| {
//             // re-mint for each iteration to avoid insufficient balance
//             token.mint(alice, alice, id, U256::from(10u64)).unwrap();
//             token
//                 .burn(
//                     black_box(alice),
//                     black_box(alice),
//                     black_box(id),
//                     black_box(U256::from(5u64)),
//                 )
//                 .unwrap();
//         });
//     });
// }
// // Core doesn’t include metadata; metadata benchmarks live in erc6909_metadata.rs

// criterion_group!(
//     core_benches,
//     bench_approve,
//     bench_transfer,
//     bench_transfer_from,
//     bench_burn,
// );
// criterion_main!(core_benches);


// benches/erc6909_core.rs THE LAST VERSION WORKING PERFECT

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stylus_sdk::testing::TestVM;
use alloy_primitives::{Address, U256};

// Core ERC-6909 implementation and its traits
use openzeppelin_stylus::token::erc6909::Erc6909;
use openzeppelin_stylus::token::erc6909::traits::{
    IErc6909,
    IErc6909Mintable,
    IErc6909Burnable,
};

/// Sets up a fresh token instance and mints initial balance for `alice`.
fn setup() -> (Erc6909, Address, Address, U256) {
    let vm = TestVM::default();
    let mut token = Erc6909::from(&vm);
    let alice = Address::new([0xAA; 20]);
    let bob = Address::new([0xBB; 20]);
    let id = U256::from(1u64);
    // Provide Alice with some initial supply to allow operations
    token.mint(alice, alice, id, U256::from(100u64)).unwrap();
    (token, alice, bob, id)
}

fn bench_approve(c: &mut Criterion) {
    let (mut token, alice, bob, id) = setup();
    c.bench_function("erc6909_core approve", |b| {
        b.iter(|| {
            token
                .approve(
                    black_box(alice),
                    black_box(bob),
                    black_box(id),
                    black_box(U256::from(50u64)),
                )
                .unwrap();
        })
    });
}

fn bench_transfer(c: &mut Criterion) {
    let (mut token, alice, bob, id) = setup();
    c.bench_function("erc6909_core transfer", |b| {
        b.iter(|| {
            // Re-mint for each iteration to maintain balance
            token.mint(alice, alice, id, U256::from(10u64)).unwrap();
            token
                .transfer(
                    black_box(alice),
                    black_box(bob),
                    black_box(id),
                    black_box(U256::from(10u64)),
                )
                .unwrap();
        })
    });
}

fn bench_transfer_from(c: &mut Criterion) {
    let (mut token, alice, bob, id) = setup();
    c.bench_function("erc6909_core transfer_from", |b| {
        b.iter(|| {
            // Reset state each iteration: mint and approve
            token.mint(alice, alice, id, U256::from(10u64)).unwrap();
            token.approve(alice, bob, id, U256::from(20u64)).unwrap();
            token
                .transfer_from(
                    black_box(bob),
                    black_box(alice),
                    black_box(bob),
                    black_box(id),
                    black_box(U256::from(5u64)),
                )
                .unwrap();
        })
    });
}

fn bench_burn(c: &mut Criterion) {
    let (mut token, alice, _, id) = setup();
    c.bench_function("erc6909_core burn", |b| {
        b.iter(|| {
            // Reset state each iteration: mint then burn
            token.mint(alice, alice, id, U256::from(10u64)).unwrap();
            token
                .burn(
                    black_box(alice),
                    black_box(alice),
                    black_box(id),
                    black_box(U256::from(5u64)),
                )
                .unwrap();
        })
    });
}

criterion_group!(
    core_benches,
    bench_approve,
    bench_transfer,
    bench_transfer_from,
    bench_burn
);
criterion_main!(core_benches);
