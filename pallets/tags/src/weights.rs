
//! Autogenerated weights for `pallet_tags`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-05-30, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `arnaud-pc`, CPU: `AMD Ryzen 3 3200G with Radeon Vega Graphics`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// target/debug/node-template
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_tags
// --extrinsic
// *
// --steps=50
// --repeat=20
// --wasm-execution=compiled
// --output
// pallets/tags/src/weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_tags`.
pub trait WeightInfo {
    fn create_tag() -> Weight;
    fn destroy_tag() -> Weight;
}

/// Weights for `pallet_tags` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    /// Storage: `TagModule::TagIndex` (r:1 w:1)
    /// Proof: `TagModule::TagIndex` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `TagModule::TagMap` (r:0 w:1)
    /// Proof: `TagModule::TagMap` (`max_values`: None, `max_size`: Some(330), added: 2805, mode: `MaxEncodedLen`)
    fn create_tag() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `6`
        //  Estimated: `1493`
        // Minimum execution time: 969_453_000 picoseconds.
        Weight::from_parts(984_382_000, 1493)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: `TagModule::TagMap` (r:1 w:1)
    /// Proof: `TagModule::TagMap` (`max_values`: None, `max_size`: Some(330), added: 2805, mode: `MaxEncodedLen`)
    fn destroy_tag() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `117`
        //  Estimated: `3795`
        // Minimum execution time: 1_031_851_000 picoseconds.
        Weight::from_parts(1_077_628_000, 3795)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
}

// For backwards compatibility and tests.
impl WeightInfo for () {
    /// Storage: `TagModule::TagIndex` (r:1 w:1)
    /// Proof: `TagModule::TagIndex` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
    /// Storage: `TagModule::TagMap` (r:0 w:1)
    /// Proof: `TagModule::TagMap` (`max_values`: None, `max_size`: Some(330), added: 2805, mode: `MaxEncodedLen`)
    fn create_tag() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `6`
        //  Estimated: `1493`
        // Minimum execution time: 969_453_000 picoseconds.
        Weight::from_parts(984_382_000, 1493)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(2_u64))
    }
    /// Storage: `TagModule::TagMap` (r:1 w:1)
    /// Proof: `TagModule::TagMap` (`max_values`: None, `max_size`: Some(330), added: 2805, mode: `MaxEncodedLen`)
    fn destroy_tag() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `117`
        //  Estimated: `3795`
        // Minimum execution time: 1_031_851_000 picoseconds.
        Weight::from_parts(1_077_628_000, 3795)
            .saturating_add(RocksDbWeight::get().reads(1_u64))
            .saturating_add(RocksDbWeight::get().writes(1_u64))
    }
}