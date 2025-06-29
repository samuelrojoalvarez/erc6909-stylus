use stylus_sdk::testing::TestVM;
use alloy_primitives::{Address, U256};

// core + the three extensions
use openzeppelin_stylus::token::erc6909::{
    Erc6909,                                    // if you need core directly
    extensions::{
        token_supply::Erc6909Supply,
        metadata_uri::Erc6909MetadataUri,
        enumerable::Erc6909Enumerable,
    },
};
use openzeppelin_stylus::token::erc6909::traits::{
    IErc6909Mintable, IErc6909Supply,
    IErc6909MetadataUri, IErc6909Enumerable,
};

fn main() {
    let vm = TestVM::default();
    let owner = Address::new([0xAA; 20]);
    let id    = U256::from(42u64);
    let amount = U256::from(100u64);
    

    // ── Supply extension ───────────────────────────────
    let mut supply = Erc6909Supply::from(&vm);
    supply.mint(owner, owner, id, amount).unwrap();
    assert_eq!(supply.total_supply(id), amount);
    println!(" ✔ total supply = {}", supply.total_supply(id));

    // ── Metadata extension ────────────────────────────
    let mut uri_ext = Erc6909MetadataUri::from(&vm);
    let uri = b"https://example.com/42".to_vec();
    uri_ext.set_token_uri(owner, id, uri.clone()).unwrap();
    assert_eq!(uri_ext.token_uri(id), uri);
    println!(" ✔ metadata URI = {:?}", uri_ext.token_uri(id));

    // ── Enumerable extension ──────────────────────────
    let mut enum_ext = Erc6909Enumerable::from(&vm);
    // we have to record that `id` was minted; in real you’d hook this in _mint
    enum_ext._record_id(id);
    assert_eq!(enum_ext.total_ids(), U256::ONE);
    assert_eq!(enum_ext.id_by_index(U256::ZERO), id);
    println!(" ✔ enumerable id[0] = {:?}", enum_ext.id_by_index(U256::ZERO));

    println!("✅ all three ERC-6909 extensions example OK");
}


