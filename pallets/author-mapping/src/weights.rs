// Copyright 2019-2022 PureStake Inc.
// This file is part of Moonbeam.

// Moonbeam is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Moonbeam is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Moonbeam.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_author_mapping
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-14, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/moonbeam
// benchmark
// --chain
// dev
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// *
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./benchmarking/frame-weight-template.hbs
// --record-proof
// --json-file
// raw.json
// --output
// ./benchmarks/

#![allow(unused_parens)]
#![allow(unused_imports)]

#[rustfmt::skip]
use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
#[rustfmt::skip]
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_author_mapping.
pub trait WeightInfo {
	#[rustfmt::skip]
	fn add_association() -> Weight;
	#[rustfmt::skip]
	fn update_association() -> Weight;
	#[rustfmt::skip]
	fn clear_association() -> Weight;
}

/// Weights for pallet_author_mapping using the Substrate node and recommended hardware.
#[rustfmt::skip]
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn add_association() -> Weight {
		(34_696_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AuthorMapping MappingWithDeposit (r:2 w:2)
	#[rustfmt::skip]
	fn update_association() -> Weight {
		(26_877_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn clear_association() -> Weight {
		(36_253_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn add_association() -> Weight {
		(34_696_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AuthorMapping MappingWithDeposit (r:2 w:2)
	#[rustfmt::skip]
	fn update_association() -> Weight {
		(26_877_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: AuthorMapping MappingWithDeposit (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	#[rustfmt::skip]
	fn clear_association() -> Weight {
		(36_253_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
