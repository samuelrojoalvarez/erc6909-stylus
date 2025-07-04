use alloy_primitives::U256;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// A minimal in-memory implementation of the enumerable API.
struct InMemoryEnumerable {
    all_ids: Vec<U256>,
}

impl InMemoryEnumerable {
    fn new() -> Self {
        Self { all_ids: Vec::new() }
    }

    fn record_id(&mut self, id: U256) {
        if !self.all_ids.contains(&id) {
            self.all_ids.push(id);
        }
    }

    fn total_ids(&self) -> U256 {
        U256::from(self.all_ids.len())
    }

    fn id_by_index(&self, index: U256) -> U256 {
        // Convert U256 index to usize by taking low 8 bytes
        let raw: [u8; 32] = index.to_le_bytes();
        let idx_bytes: [u8; 8] =
            raw[0..8].try_into().expect("slice with incorrect length");
        let idx = u64::from_le_bytes(idx_bytes) as usize;
        if idx < self.all_ids.len() {
            self.all_ids[idx]
        } else {
            U256::ZERO
        }
    }
}

fn setup() -> InMemoryEnumerable {
    let mut ext = InMemoryEnumerable::new();
    for i in 1..=100u64 {
        ext.record_id(U256::from(i));
    }
    ext
}

fn bench_total_ids(c: &mut Criterion) {
    let ext = setup();
    c.bench_function("erc6909_enum total_ids (100)", |b| {
        b.iter(|| {
            black_box(ext.total_ids());
        })
    });
}

fn bench_id_by_index(c: &mut Criterion) {
    let ext = setup();
    c.bench_function("erc6909_enum id_by_index (50)", |b| {
        b.iter(|| {
            // Convert U256 index to usize by taking low 8 bytes
            let raw: [u8; 32] = black_box(U256::from(50)).to_le_bytes();
            let idx_bytes: [u8; 8] = raw[0..8].try_into().unwrap();
            let idx = u64::from_le_bytes(idx_bytes) as usize;
            black_box(ext.id_by_index(U256::from(idx as u64)));
        })
    });
}

criterion_group!(enum_benches, bench_total_ids, bench_id_by_index);
criterion_main!(enum_benches);
