#![no_std]

use alloc::{vec, vec::Vec};
use alloy_primitives::U256;
use stylus_sdk::{
    prelude::{storage, HostAccess, StorageType},
    host::VM,
};
use stylus_sdk::storage::{StorageVec, StorageMap, StorageUint};

use crate::token::erc6909::traits::IErc6909Enumerable;
use core::convert::TryInto;

/// Enumerable extension: tracks all unique IDs and their indices.
#[storage]
pub struct Erc6909Enumerable {
    /// List of token IDs in insertion order.
    all_ids: StorageVec<StorageUint<256, 4>>,
    /// Mapping from token ID → its index in `all_ids`.
    index_of: StorageMap<U256, StorageUint<256, 4>>,
}

impl Erc6909Enumerable {
    /// Internal hook: record a new ID if not seen before.
    pub fn _record_id(&mut self, id: U256) {
        // Skip if already recorded
        let exists_u256: U256 = self.index_of.get(id).into();
        // If already recorded (non-zero), skip
        if exists_u256 != U256::ZERO {
            return;
        }
        
        // Determine next index
        let next_index: U256 = U256::from(self.all_ids.len());
        
        // Append to list and update map
        self.all_ids.push(id);
        self.index_of.insert(id, next_index);
    }
}

impl IErc6909Enumerable for Erc6909Enumerable {
    /// Returns the total number of unique IDs recorded.
    fn total_ids(&self) -> U256 {
        U256::from(self.all_ids.len())
    }

    /// Returns the token ID at the given index, or zero if out of bounds.
    fn id_by_index(&self, index: U256) -> U256 {
        // Convert U256 index to usize by taking the low 8 bytes
        let raw: [u8; 32] = index.to_le_bytes();
        let idx_u64 = u64::from_le_bytes(raw[0..8].try_into().unwrap());
        let idx_usize = idx_u64 as usize;

        self.all_ids
            .get(idx_usize)
            .unwrap_or(U256::ZERO)
    }
}
