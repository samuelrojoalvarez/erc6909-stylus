use alloy_primitives::{Address, U256};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
// Import the supply extension and mint/burn traits
use openzeppelin_stylus::token::erc6909::extensions::token_supply::Erc6909Supply;
use openzeppelin_stylus::token::erc6909::traits::{
    IErc6909Burnable, IErc6909Mintable, IErc6909Supply,
};
use stylus_sdk::testing::TestVM;

/// Returns (token, caller, recipient, token_id)
fn setup() -> (Erc6909Supply, Address, Address, U256) {
    let vm = TestVM::default();
    let token = Erc6909Supply::from(&vm);
    let caller = Address::new([0xAA; 20]);
    let recipient = Address::new([0xBB; 20]);
    let id = U256::from(1u64);
    (token, caller, recipient, id)
}

fn bench_supply_mint(c: &mut Criterion) {
    let (mut token, caller, recipient, id) = setup();
    c.bench_function("erc6909_supply mint", |b| {
        b.iter(|| {
            token
                .mint(
                    black_box(caller),
                    black_box(recipient),
                    black_box(id),
                    black_box(U256::from(10u64)),
                )
                .unwrap();
        })
    });
}

fn bench_supply_burn(c: &mut Criterion) {
    let (mut token, caller, recipient, id) = setup();
    c.bench_function("erc6909_supply burn", |b| {
        b.iter(|| {
            // re-mint then burn each iteration to avoid balance depletion
            token.mint(caller, recipient, id, U256::from(20u64)).unwrap();
            token
                .burn(
                    black_box(caller),
                    black_box(recipient),
                    black_box(id),
                    black_box(U256::from(5u64)),
                )
                .unwrap();
        })
    });
}

fn bench_supply_total(c: &mut Criterion) {
    let (mut token, caller, recipient, id) = setup();
    // Pre-mint supply
    token.mint(caller, recipient, id, U256::from(123u64)).unwrap();
    c.bench_function("erc6909_supply total_supply", |b| {
        b.iter(|| {
            black_box(token.total_supply(black_box(id)));
        })
    });
}

criterion_group!(
    supply_benches,
    bench_supply_mint,
    bench_supply_burn,
    bench_supply_total
);
criterion_main!(supply_benches);
