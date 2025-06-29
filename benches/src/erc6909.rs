// benches/src/erc6909.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stylus_sdk::testing::TestVM;
use alloy_primitives::{Address, U256};
use openzeppelin_stylus::token::erc6909::{
    extensions::{token_supply::Erc6909Supply, metadata_uri::Erc6909MetadataUri},
    traits::{IErc6909, IErc6909Mintable, IErc6909Burnable, IErc6909Supply, IErc6909MetadataUri},
};

fn bench_mint(c: &mut Criterion) {
    let vm = TestVM::default();
    let mut token = Erc6909Supply::from(&vm);
    let alice = Address::new([0xAA; 20]);
    c.bench_function("erc6909 mint", |b| {
        b.iter(|| {
            token
                .mint(
                    black_box(alice),
                    black_box(U256::from(1u64)),
                    black_box(U256::from(1u64)),
                )
                .unwrap();
        })
    });
}

fn bench_approve(c: &mut Criterion) {
    let vm = TestVM::default();
    let mut token = Erc6909Supply::from(&vm);
    let owner   = Address::new([0xAA; 20]);
    let spender = Address::new([0xBB; 20]);
    let id      = U256::from(1u64);
    c.bench_function("erc6909 approve", |b| {
        b.iter(|| {
            token
                .approve(
                    black_box(owner),
                    black_box(spender),
                    black_box(id),
                    black_box(U256::from(1u64)),
                )
                .unwrap();
        })
    });
}

fn bench_transfer(c: &mut Criterion) {
    let vm = TestVM::default();
    let mut token = Erc6909Supply::from(&vm);
    let alice = Address::new([0xAA; 20]);
    let bob   = Address::new([0xBB; 20]);
    let id    = U256::from(1u64);
    // seed one token so the transfer will actually do work
    token.mint(alice, id, U256::from(1u64)).unwrap();
    c.bench_function("erc6909 transfer", |b| {
        b.iter(|| {
            token
                .transfer(
                    black_box(alice),
                    black_box(bob),
                    black_box(id),
                    black_box(U256::from(1u64)),
                )
                .unwrap();
        })
    });
}

fn bench_transfer_from(c: &mut Criterion) {
    let vm = TestVM::default();
    let mut token = Erc6909Supply::from(&vm);
    let alice = Address::new([0xAA; 20]);
    let bob   = Address::new([0xBB; 20]);
    let id    = U256::from(1u64);
    token.mint(alice, id, U256::from(1u64)).unwrap();
    token.approve(alice, bob, id, U256::from(1u64)).unwrap();
    c.bench_function("erc6909 transfer_from", |b| {
        b.iter(|| {
            token
                .transfer_from(
                    black_box(bob),
                    black_box(alice),
                    black_box(bob),
                    black_box(id),
                    black_box(U256::from(1u64)),
                )
                .unwrap();
        })
    });
}

fn bench_burn(c: &mut Criterion) {
    let vm = TestVM::default();
    let mut token = Erc6909Supply::from(&vm);
    let alice = Address::new([0xAA; 20]);
    let id    = U256::from(1u64);
    token.mint(alice, id, U256::from(1u64)).unwrap();
    c.bench_function("erc6909 burn", |b| {
        b.iter(|| {
            token
                .burn(
                    black_box(alice),
                    black_box(id),
                    black_box(U256::from(1u64)),
                )
                .unwrap();
        })
    });
}

fn bench_set_token_uri(c: &mut Criterion) {
    let vm = TestVM::default();
    let mut meta = Erc6909MetadataUri::from(&vm);
    let caller = Address::new([0xAA; 20]);
    let id     = U256::from(1u64);
    let uri    = b"https://token/1".to_vec();
    c.bench_function("erc6909 set_token_uri", |b| {
        b.iter(|| {
            meta
                .set_token_uri(
                    black_box(caller),
                    black_box(id),
                    black_box(uri.clone()),
                )
                .unwrap();
        })
    });
}

fn bench_token_uri(c: &mut Criterion) {
    let vm = TestVM::default();
    let mut meta = Erc6909MetadataUri::from(&vm);
    let caller = Address::new([0xAA; 20]);
    let id     = U256::from(1u64);
    let uri    = b"https://token/1".to_vec();
    // pre-seed one URI so reads actually traverse storage
    meta.set_token_uri(caller, id, uri.clone()).unwrap();
    c.bench_function("erc6909 token_uri", |b| {
        b.iter(|| {
            let _ = meta.token_uri(black_box(id));
        })
    });
}

// If you implement an enumerable extension, add two more benches here:
// fn bench_total_ids(c: &mut Criterion) { … }
// fn bench_id_by_index(c: &mut Criterion) { … }

criterion_group!(
    erc6909_benches,
    bench_mint,
    bench_approve,
    bench_transfer,
    bench_transfer_from,
    bench_burn,
    bench_set_token_uri,
    bench_token_uri,
);
criterion_main!(erc6909_benches);
