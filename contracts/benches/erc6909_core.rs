// benches/erc6909_core.rs  LV 

use alloy_primitives::{Address, U256};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use openzeppelin_stylus::token::erc6909::traits::{
    IErc6909, IErc6909Burnable, IErc6909Mintable,
};
// Core ERC-6909 implementation and its traits
use openzeppelin_stylus::token::erc6909::Erc6909;
use stylus_sdk::testing::TestVM;

/// Sets up a fresh token instance and mints initial balance for `owner`.
fn setup() -> (Erc6909, Address, Address, U256) {
    let vm = TestVM::default();
    let mut token = Erc6909::from(&vm);
    let owner = Address::new([0xAA; 20]);
    let recipient = Address::new([0xBB; 20]);
    let id = U256::from(1u64);
    // Provide Owner with some initial supply to allow operations
    token.mint(owner, owner, id, U256::from(100u64)).unwrap();
    (token, owner, recipient, id)
}

fn bench_approve(c: &mut Criterion) {
    let (mut token, owner, recipient, id) = setup();
    c.bench_function("erc6909_core approve", |b| {
        b.iter(|| {
            token
                .approve(
                    black_box(owner),
                    black_box(recipient),
                    black_box(id),
                    black_box(U256::from(50u64)),
                )
                .unwrap();
        })
    });
}

fn bench_transfer(c: &mut Criterion) {
    let (mut token, owner, recipient, id) = setup();
    c.bench_function("erc6909_core transfer", |b| {
        b.iter(|| {
            // Re-mint for each iteration to maintain balance
            token.mint(owner, owner, id, U256::from(10u64)).unwrap();
            token
                .transfer(
                    black_box(owner),
                    black_box(recipient),
                    black_box(id),
                    black_box(U256::from(10u64)),
                )
                .unwrap();
        })
    });
}

fn bench_transfer_from(c: &mut Criterion) {
    let (mut token, owner, recipient, id) = setup();
    c.bench_function("erc6909_core transfer_from", |b| {
        b.iter(|| {
            // Reset state each iteration: mint and approve
            token.mint(owner, owner, id, U256::from(10u64)).unwrap();
            token.approve(owner, recipient, id, U256::from(20u64)).unwrap();
            token
                .transfer_from(
                    black_box(recipient),
                    black_box(owner),
                    black_box(recipient),
                    black_box(id),
                    black_box(U256::from(5u64)),
                )
                .unwrap();
        })
    });
}

fn bench_burn(c: &mut Criterion) {
    let (mut token, owner, _, id) = setup();
    c.bench_function("erc6909_core burn", |b| {
        b.iter(|| {
            // Reset state each iteration: mint then burn
            token.mint(owner, owner, id, U256::from(10u64)).unwrap();
            token
                .burn(
                    black_box(owner),
                    black_box(owner),
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

