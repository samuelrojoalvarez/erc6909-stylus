use openzeppelin_stylus::token::erc6909::{
    error::Error,
    traits::{IErc6909Supply, IErc6909MetadataUri, IErc6909Enumerable, IErc6909Mintable, IErc6909Burnable},
    extensions::{
        token_supply::Erc6909Supply,
        metadata_uri::Erc6909MetadataUri,
        enumerable::Erc6909Enumerable,
    },
};

use stylus_sdk::testing::TestVM;
use alloy_primitives::{Address, U256};

use proptest::prelude::*;

proptest! {
    /// Randomized property test for the Supply extension.
    #[motsu::test]
    fn supply_mint_burn_roundtrip(id in any::<u64>(), amt in any::<u64>()) {
        let vm = TestVM::default();
        let id = U256::from(id);
        let owner = Address::new([0xAA; 20]);

        // 1) Mint
        let mut sup = Erc6909Supply::from(&vm);
        sup.mint(owner, owner, id, U256::from(amt)).unwrap();
        prop_assert_eq!(sup.total_supply(id), U256::from(amt));

        // 2) Burn the same amount
        sup.burn(owner, owner, id, U256::from(amt)).unwrap();
        prop_assert_eq!(sup.total_supply(id), U256::ZERO);
    }

    /// Randomized property test for Metadata URI round-trip.
    #[motsu::test]
    fn metadata_uri_roundtrip(id in any::<u64>()) {
        let vm = TestVM::default();
        let id = U256::from(id);
        let recipient = Address::new([0xBB; 20]);
        let uri_bytes = b"https://example.com/".iter()
            .chain(id.to_string().as_bytes())
            .cloned()
            .collect::<Vec<u8>>();

        let mut meta = Erc6909MetadataUri::from(&vm);
        prop_assert_eq!(meta.set_token_uri(recipient, id, uri_bytes.clone()), Ok(true));
        prop_assert_eq!(meta.token_uri(id), uri_bytes);
    }

    /// Randomized property test for Enumerable extension.
    #[motsu::test]
    fn enumerable_tracks_ids(ids in prop::collection::vec(any::<u64>(), 1..10)) {
        let vm = TestVM::default();
        let mut en = Erc6909Enumerable::from(&vm);

        // insert each unique
        let mut seen = std::collections::BTreeSet::new();
        for raw in &ids {
            let id = U256::from(*raw);
            en._record_id(id);
            seen.insert(id);
        }

        // total_ids must match unique count
        prop_assert_eq!(en.total_ids(), U256::from(seen.len()));

        // and id_by_index must return each in insertion order
        for (i, &id) in seen.iter().enumerate() {
            prop_assert_eq!(en.id_by_index(U256::from(i as u64)), id);
        }
    }

    #[motsu::test]
    fn mint_burn_roundtrip(id in any::<u64>(), amt in 0u64..1_000u64, owner_bytes in any::<[u8;20]>()) {
        let vm    = TestVM::default();
        let mut sup = Erc6909Supply::from(&vm);

        let id = U256::from(id);
        let amt = U256::from(amt);
        let owner = alloy_primitives::Address::from(owner_bytes);

        // start from zero
        prop_assume!(sup.total_supply(id) == U256::ZERO);

        // mint then burn
        sup.mint(owner, owner, id, amt).unwrap();
        sup.burn(owner, owner, id, amt).unwrap();

        // total_supply should be zero again
        prop_assert_eq!(sup.total_supply(id), U256::ZERO);
    }
}
