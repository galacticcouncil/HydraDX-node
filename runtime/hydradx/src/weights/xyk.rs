// This file is part of HydraDX.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for `pallet_xyk`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-15, STEPS: `10`, REPEAT: `30`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bench-bot`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: 1024

// Executed Command:
// target/release/hydradx
// benchmark
// pallet
// --chain=dev
// --steps=10
// --repeat=30
// --wasm-execution=compiled
// --heap-pages=4096
// --template=.maintain/pallet-weight-template-no-back.hbs
// --pallet=pallet_xyk
// --output=./weights/xyk.rs
// --extrinsic=*

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

use pallet_xyk::weights::WeightInfo;

/// Weight functions for `pallet_xyk`.
pub struct HydraWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for HydraWeight<T> {
	/// Storage: `LBP::PoolData` (r:1 w:0)
	/// Proof: `LBP::PoolData` (`max_values`: None, `max_size`: Some(163), added: 2638, mode: `MaxEncodedLen`)
	/// Storage: `XYK::ShareToken` (r:1 w:1)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:7)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::AssetIds` (r:1 w:1)
	/// Proof: `AssetRegistry::AssetIds` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::NextAssetId` (r:1 w:1)
	/// Proof: `AssetRegistry::NextAssetId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:4 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:1)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:3 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencies` (r:3 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencies` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `XYK::TotalLiquidity` (r:0 w:1)
	/// Proof: `XYK::TotalLiquidity` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `XYK::PoolAssets` (r:0 w:1)
	/// Proof: `XYK::PoolAssets` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn create_pool() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3522`
		//  Estimated: `19071`
		// Minimum execution time: 588_643_000 picoseconds.
		Weight::from_parts(592_313_000, 19071)
			.saturating_add(T::DbWeight::get().reads(33))
			.saturating_add(T::DbWeight::get().writes(20))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:7)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `XYK::TotalLiquidity` (r:1 w:1)
	/// Proof: `XYK::TotalLiquidity` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:1 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:4 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:4 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn add_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4159`
		//  Estimated: `19071`
		// Minimum execution time: 353_004_000 picoseconds.
		Weight::from_parts(354_898_000, 19071)
			.saturating_add(T::DbWeight::get().reads(27))
			.saturating_add(T::DbWeight::get().writes(14))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:1)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `XYK::TotalLiquidity` (r:1 w:1)
	/// Proof: `XYK::TotalLiquidity` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:7 w:7)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:1)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:4 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:2 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:3)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::TotalIssuance` (r:1 w:1)
	/// Proof: `Tokens::TotalIssuance` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// Storage: `XYK::PoolAssets` (r:0 w:1)
	/// Proof: `XYK::PoolAssets` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	fn remove_liquidity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4208`
		//  Estimated: `19071`
		// Minimum execution time: 523_229_000 picoseconds.
		Weight::from_parts(525_459_000, 19071)
			.saturating_add(T::DbWeight::get().reads(29))
			.saturating_add(T::DbWeight::get().writes(18))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:6 w:6)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn sell() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4057`
		//  Estimated: `16488`
		// Minimum execution time: 306_824_000 picoseconds.
		Weight::from_parts(308_832_000, 16488)
			.saturating_add(T::DbWeight::get().reads(24))
			.saturating_add(T::DbWeight::get().writes(11))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:6 w:6)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	fn buy() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4057`
		//  Estimated: `16488`
		// Minimum execution time: 305_679_000 picoseconds.
		Weight::from_parts(306_896_000, 16488)
			.saturating_add(T::DbWeight::get().reads(24))
			.saturating_add(T::DbWeight::get().writes(11))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:6 w:6)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 2]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_sell(c: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1187 + e * (2870 ±0)`
		//  Estimated: `6156 + e * (10332 ±52_268_479_977_062_104)`
		// Minimum execution time: 25_878_000 picoseconds.
		Weight::from_parts(9_649_436, 6156)
			// Standard Error: 80_766
			.saturating_add(Weight::from_parts(8_443_704, 0).saturating_mul(c.into()))
			// Standard Error: 80_766
			.saturating_add(Weight::from_parts(289_624_454, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((21_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes((11_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 10332).saturating_mul(e.into()))
	}
	/// Storage: `XYK::ShareToken` (r:1 w:0)
	/// Proof: `XYK::ShareToken` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Tokens::Accounts` (r:6 w:6)
	/// Proof: `Tokens::Accounts` (`max_values`: None, `max_size`: Some(108), added: 2583, mode: `MaxEncodedLen`)
	/// Storage: `Duster::AccountBlacklist` (r:2 w:0)
	/// Proof: `Duster::AccountBlacklist` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::BannedAssets` (r:3 w:0)
	/// Proof: `AssetRegistry::BannedAssets` (`max_values`: None, `max_size`: Some(20), added: 2495, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::Assets` (r:3 w:0)
	/// Proof: `AssetRegistry::Assets` (`max_values`: None, `max_size`: Some(125), added: 2600, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AccountCurrencyMap` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AccountCurrencyMap` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `MultiTransactionPayment::AcceptedCurrencyPrice` (r:1 w:0)
	/// Proof: `MultiTransactionPayment::AcceptedCurrencyPrice` (`max_values`: None, `max_size`: Some(28), added: 2503, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:3 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `AssetRegistry::ExistentialDepositCounter` (r:1 w:1)
	/// Proof: `AssetRegistry::ExistentialDepositCounter` (`max_values`: Some(1), `max_size`: Some(16), added: 511, mode: `MaxEncodedLen`)
	/// Storage: `EmaOracle::Accumulator` (r:1 w:1)
	/// Proof: `EmaOracle::Accumulator` (`max_values`: Some(1), `max_size`: Some(5921), added: 6416, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 3]`.
	/// The range of component `e` is `[0, 1]`.
	fn router_execution_buy(c: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1187 + e * (2870 ±0)`
		//  Estimated: `6156 + e * (10332 ±0)`
		// Minimum execution time: 33_714_000 picoseconds.
		Weight::from_parts(11_014_445, 6156)
			// Standard Error: 41_799
			.saturating_add(Weight::from_parts(7_776_697, 0).saturating_mul(c.into()))
			// Standard Error: 70_916
			.saturating_add(Weight::from_parts(287_790_853, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().reads((21_u64).saturating_mul(e.into())))
			.saturating_add(T::DbWeight::get().writes((11_u64).saturating_mul(e.into())))
			.saturating_add(Weight::from_parts(0, 10332).saturating_mul(e.into()))
	}

	fn calculate_spot_price() -> Weight {
		Weight::zero()
	}
}
