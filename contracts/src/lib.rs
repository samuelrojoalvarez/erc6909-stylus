#![allow(
    clippy::module_name_repetitions,
    clippy::used_underscore_items,
    deprecated
)]
#![cfg_attr(not(any(test, feature = "export-abi")), no_std, no_main)]
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]
#![deny(rustdoc::broken_intra_doc_links)]
extern crate alloc;

pub mod access;
pub mod finance;
pub mod token;
pub mod utils;
