// Copyright 2019-2022 ChainX Project Authors. Licensed under GPL-3.0.

//! Weights for xpallet_dex_spot
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-05-13, STEPS: 50, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("benchmarks"), DB CACHE: 1024

// Executed Command:
// ./target/release/chainx
// benchmark
// --chain=benchmarks
// --steps=50
// --repeat=20
// --pallet=xpallet_dex_spot
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./xpallets/dex/spot/src/weights.rs
// --template=./scripts/xpallet-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
    traits::Get,
    weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for xpallet_dex_spot.
pub trait WeightInfo {
    fn put_order() -> Weight;
    fn cancel_order() -> Weight;
    fn force_cancel_order() -> Weight;
    fn set_handicap() -> Weight;
    fn set_price_fluctuation() -> Weight;
    fn add_trading_pair() -> Weight;
    fn update_trading_pair() -> Weight;
}

/// Weights for xpallet_dex_spot using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn put_order() -> Weight {
        (142_883_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(11 as Weight))
            .saturating_add(T::DbWeight::get().writes(6 as Weight))
    }
    fn cancel_order() -> Weight {
        (133_946_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(8 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    fn force_cancel_order() -> Weight {
        (128_033_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(8 as Weight))
            .saturating_add(T::DbWeight::get().writes(5 as Weight))
    }
    fn set_handicap() -> Weight {
        (8_101_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn set_price_fluctuation() -> Weight {
        (19_612_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    fn add_trading_pair() -> Weight {
        (38_706_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(3 as Weight))
    }
    fn update_trading_pair() -> Weight {
        (32_363_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn put_order() -> Weight {
        (142_883_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(11 as Weight))
            .saturating_add(RocksDbWeight::get().writes(6 as Weight))
    }
    fn cancel_order() -> Weight {
        (133_946_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(8 as Weight))
            .saturating_add(RocksDbWeight::get().writes(5 as Weight))
    }
    fn force_cancel_order() -> Weight {
        (128_033_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(8 as Weight))
            .saturating_add(RocksDbWeight::get().writes(5 as Weight))
    }
    fn set_handicap() -> Weight {
        (8_101_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn set_price_fluctuation() -> Weight {
        (19_612_000 as Weight).saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
    fn add_trading_pair() -> Weight {
        (38_706_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(2 as Weight))
            .saturating_add(RocksDbWeight::get().writes(3 as Weight))
    }
    fn update_trading_pair() -> Weight {
        (32_363_000 as Weight)
            .saturating_add(RocksDbWeight::get().reads(1 as Weight))
            .saturating_add(RocksDbWeight::get().writes(1 as Weight))
    }
}
