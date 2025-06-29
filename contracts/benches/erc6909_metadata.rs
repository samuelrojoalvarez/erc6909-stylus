// benches/erc6909_metadata.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use stylus_sdk::testing::TestVM;
use alloy_primitives::{Address, U256};

// point at your library crate, not `crate::…`
use openzeppelin_stylus::token::erc6909::extensions::metadata_uri::Erc6909MetadataUri;
use openzeppelin_stylus::token::erc6909::traits::IErc6909MetadataUri;

fn setup() -> (Erc6909MetadataUri, Address, U256) {
    let vm = TestVM::default();
    let ext = Erc6909MetadataUri::from(&vm);
    let caller = Address::new([0xAA; 20]);
    let id = U256::from(1u64);
    (ext, caller, id)
}

fn bench_set_uri_100b(c: &mut Criterion) {
    let (mut ext, caller, id) = setup();
    let uri = vec![b'a'; 100];
    c.bench_function("erc6909_metadata set_token_uri (100 B)", |b| {
        b.iter(|| {
            ext.set_token_uri(
                black_box(caller),
                black_box(id),
                black_box(uri.clone()),
            )
            .unwrap();
        })
    });
}

fn bench_get_uri_100b(c: &mut Criterion) {
    let (mut ext, caller, id) = setup();
    let uri = vec![b'b'; 100];
    ext.set_token_uri(caller, id, uri.clone()).unwrap();
    c.bench_function("erc6909_metadata token_uri (100 B)", |b| {
        b.iter(|| {
            black_box(ext.token_uri(black_box(id)));
        })
    });
}

criterion_group!(metadata_benches, bench_set_uri_100b, bench_get_uri_100b);
criterion_main!(metadata_benches);
