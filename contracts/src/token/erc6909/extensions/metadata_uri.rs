#![no_std]
extern crate alloc;

use alloc::{vec, vec::Vec};
use core::convert::TryFrom;

use alloy_primitives::{Address, Uint, U256};
use stylus_sdk::{
    prelude::{storage, HostAccess, StorageType},
    storage::{StorageBool, StorageMap, StorageUint},
};

use crate::token::erc6909::{traits::IErc6909MetadataUri, Error};

/// ERC-6909 metadata-URI extension: stores for each `id`
///   • `uri_len[id]` → length (Uint<256>),  
///   • `uri_byte[id][idx]` → the byte at `idx` (Uint<8>).
#[storage]
pub struct Erc6909MetadataUri {
    /// id → length
    uri_len: StorageMap<U256, StorageUint<256, 4>>,
    /// id → (index → byte)
    uri_byte: StorageMap<U256, StorageMap<U256, StorageUint<8, 1>>>,
}

impl IErc6909MetadataUri for Erc6909MetadataUri {
    fn token_uri(&self, id: U256) -> Vec<u8> {
        // 1) read the length
        let len_u256 = self.uri_len.get(id);

        // 2) iterate in U256 space
        let mut idx = U256::ZERO;
        let mut out = Vec::new();
        while idx < len_u256 {
            // read the stored Uint<8>
            let byte_u8 = {
                let b_u256 = self
                    .uri_byte
                    .get(id) // get the second‐level map
                    .get(idx); // get the byte at `idx`
                               // convert Uint<8,1> → u8 via TryFrom
                u8::try_from(b_u256).unwrap_or(0)
            };
            out.push(byte_u8);
            idx = idx.checked_add(U256::ONE).unwrap();
        }
        out
    }

    fn set_token_uri(
        &mut self,
        caller: Address,
        id: U256,
        uri: Vec<u8>,
    ) -> Result<bool, Error> {
        if caller.is_zero() {
            return Err(Error::InvalidApprover);
        }

        // 1) store the length
        let len = U256::from(uri.len() as u64);
        self.uri_len.insert(id, len);

        // 2) store each byte
        let mut idx = U256::ZERO;
        for &b in uri.iter() {
            // convert raw u8 → Uint<8,1>
            let byte8: Uint<8, 1> = Uint::from_be_bytes([b]);
            // let b_u256 = U256::from(b as u64);

            // nested setter: uri_byte[id][idx] = b_u256
            let mut by_id = self.uri_byte.setter(id);
            let mut by_idx = by_id.setter(idx);
            by_idx.set(byte8);
            idx = idx.checked_add(U256::ONE).unwrap();
        }
        Ok(true)
    }
}

