# Draft PR: ERC-6909 Multi‐Token Standard Implementation in Rust

## Overview

This draft PR introduces a Rust implementation of EIP‑6909 (“Single‑ID Multi‑Token Standard”) in the Stylus SDK, including three key components:

- **Core (ERC6909):** Balance and allowance management without supply tracking.

- **Supply Extension (ERC6909Supply):** Per‑ID total supply tracking.

- **Metadata Extension (ERC6909MetadataUri):** On‑chain storage of per‑ID URIs.

- **Enumerable Extension (ERC6909Enumerable):** Recording and iteration of all seen token IDs.

A composite `MyToken` aggregates these components into a single contract.

## Design Goals

**1. Gas/Efficiency:** Leverage `#[storage]`‐generated accessors, minimal loops, and checked arithmetic.

**2. Modularity:** Core + orthogonal extensions that can be mixed & matched.

**3. Rust‐Native Ergonomics:** Clear traits (`IErc6909`, `IErc6909Supply`, etc.), no `unsafe`, and no‐std compatibility.

**4. Testability:** Use `stylus_test::TestVM` for isolated unit tests; migrate to `motsu::test` for WASM execution.

**5. Extensibility:** Future extensions (e.g. Role, Freeze) can follow the same pattern.

## Architecture

![image](https://github.com/user-attachments/assets/71022608-7b00-4ed3-8727-5041f7e88696)



**Core:** Erc6909  **-->** **`$pwd`:** [/contracts/src/token/erc6909/erc6909.rs](https://github.com/samuelrojoalvarez/erc6909-stylus/blob/main/contracts/src/token/erc6909/erc6909.rs)

- **StorageLayout:** 

    - `balances: Map<Owner, Map<Id, Uint<256>>>`

    - `allowances: Map<Owner, Map<Spender, Map<Id, Uint<256>>>>`

    - `operator_approvals: Map<Owner, Map<Operator, bool>>`

- **API (trait IErc6909)**:

    - `balance_of(owner, id)`

    - `allowance(owner, spender, id)`

    - `is_operator(owner, operator)`

    - `approve(caller, spender, id, amount)`

    - `set_operator(caller, operator, approved)`

    - `transfer(caller, to, id, amount)`

    - `transfer_from(caller, from, to, id, amount)`

- **Internal Hooks:** `_update`, `_mint`, `_burn` for transfers, minting, and burning.

**Supply Extension:** Erc6909Supply **-->** **`$pwd`:** [/contracts/src/token/erc6909/extensions/token_supply.rs](https://github.com/samuelrojoalvarez/erc6909-stylus/blob/main/contracts/src/token/erc6909/extensions/token_supply.rs)

  - **Tracks** `_total_supplies: Map<Id, Uint<256>>`

  - **Overrides** the core `_update` to increment/decrement on mint/burn.

  - **Trait:** `IErc6909Supply` with `total_supply(id)`.

**Metadata Extension:** Erc6909MetadataUri  **-->** **`$pwd`:** [/contracts/src/token/erc6909/extensions/metadata_uri.rs](https://github.com/samuelrojoalvarez/erc6909-stylus/blob/main/contracts/src/token/erc6909/extensions/metadata_uri.rs)

- **Stores:**

    - `uri_len: Map<Id, Uint<256>>`

    - `uri_byte: Map<Id, Map<Idx, Uint<8>>>`

- **API (trait IErc6909MetadataUri):** 

    - `token_uri(id): Vec<u8>`

    - `set_token_uri(caller, id, uri)`

**Enumerable Extension:** Erc6909Enumerable **-->** **`$pwd`:** [/contracts/src/token/erc6909/extensions/enumerable.rs](https://github.com/samuelrojoalvarez/erc6909-stylus/blob/main/contracts/src/token/erc6909/extensions/enumerable.rs)

- **Stores:**

    - `all_ids: Vec<Uint<256>>`

    - `index_of: Map<Id, Uint<256>>`

  - **Behavior:** `_record_id(id)` on first mint; trait `IErc6909Enumerable` with `total_ids()` & `id_by_index()`.

## **Composite Token:** `MyToken`

   ```bash

    #[storage]
    struct MyToken { core: Erc6909, supply: Erc6909Supply, metadata: Erc6909MetadataUri, enumerable: Erc6909Enumerable }
   ```
    
- Delegates calls to the appropriate extension.
        
- Single `from(&vm)` constructor wiring all sub‐contracts.
    

## **Testing Strategy**

- **Unit Tests (Native):** -->  **`$pwd`:** [/contracts/src/token/erc6909/erc6909.rs](https://github.com/samuelrojoalvarez/erc6909-stylus/blob/main/contracts/src/token/erc6909/erc6909.rs).
  
`#[cfg(test)]` modules driven by `TestVM`, using `motsu::test` for consistency.

- **Examples & Integration:** **`$pwd`:** [/contracts/examples/](https://github.com/samuelrojoalvarez/erc6909-stylus/tree/main/contracts/examples) :
     ```bash
    total 16
    drwxrwxr-x 2 ubu ubu 4096 jun 27 23:19 .
    drwxrwxr-x 8 ubu ubu 4096 jun 28 03:08 ..
    -rw-rw-r-- 1 ubu ubu 1923 jun 27 23:19 erc6909-extensions.rs
    -rw-rw-r-- 1 ubu ubu 1994 jun 28 03:48 erc6909-supply.rs
   ```

- **Benchmarks:** **`$pwd`:** [/contracts/benches/](https://github.com/samuelrojoalvarez/erc6909-stylus/tree/main/contracts/benches) : 

   ```bash
    total 36
    drwxrwxr-x 2 ubu ubu  4096 jun 26 14:19 .
    drwxrwxr-x 8 ubu ubu  4096 jun 28 03:08 ..
    -rw-rw-r-- 1 ubu ubu 13152 jun 26 18:52 erc6909_core.rs
    -rw-rw-r-- 1 ubu ubu  1980 jun 28 00:05 erc6909_enumerable.rs
    -rw-rw-r-- 1 ubu ubu  1455 jun 28 00:05 erc6909_metadata.rs
    -rw-rw-r-- 1 ubu ubu  2224 jun 28 00:05 erc6909_supply.rs
   ```

## **Next Steps & Discussion**

- **WASM Test Harness:** Migrate to wasm32-wasi + motsu for on‐chain parity.

- **Documentation:** Flesh out docs/EIP-6909.md with storage layout comparisons vs. Solidity.

- **Security Review:** Edge cases for overflow, large URI sizes, and reentrancy.
