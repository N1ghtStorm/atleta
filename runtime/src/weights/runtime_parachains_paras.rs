// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `runtime_parachains::paras`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/atleta-node
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=runtime_parachains::paras
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/atleta/src/weights/runtime_parachains_paras.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `runtime_parachains::paras`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_parachains::paras::WeightInfo for WeightInfo<T> {
	/// Storage: Paras CurrentCodeHash (r:1 w:1)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHashRefs (r:1 w:1)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras PastCodeMeta (r:1 w:1)
	/// Proof Skipped: Paras PastCodeMeta (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras PastCodePruning (r:1 w:1)
	/// Proof Skipped: Paras PastCodePruning (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PastCodeHash (r:0 w:1)
	/// Proof Skipped: Paras PastCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:0 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 3145728]`.
	fn force_set_current_code(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `8309`
		//  Estimated: `11774`
		// Minimum execution time: 31_941_000 picoseconds.
		Weight::from_parts(32_139_000, 0)
			.saturating_add(Weight::from_parts(0, 11774))
			// Standard Error: 5
			.saturating_add(Weight::from_parts(2_011, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Paras Heads (r:0 w:1)
	/// Proof Skipped: Paras Heads (max_values: None, max_size: None, mode: Measured)
	/// The range of component `s` is `[1, 1048576]`.
	fn force_set_current_head(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_275_000 picoseconds.
		Weight::from_parts(8_321_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(858, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Paras Heads (r:0 w:1)
	fn force_set_most_recent_context() -> Weight {
		Weight::from_parts(10_155_000, 0)
			// Standard Error: 0
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeHash (r:1 w:1)
	/// Proof Skipped: Paras FutureCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CurrentCodeHash (r:1 w:0)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeCooldowns (r:1 w:1)
	/// Proof Skipped: Paras UpgradeCooldowns (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:1 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteList (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras CodeByHashRefs (r:1 w:1)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeRestrictionSignal (r:0 w:1)
	/// Proof Skipped: Paras UpgradeRestrictionSignal (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 3145728]`.
	fn force_schedule_code_upgrade(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `8715`
		//  Estimated: `12180`
		// Minimum execution time: 49_923_000 picoseconds.
		Weight::from_parts(50_688_000, 0)
			.saturating_add(Weight::from_parts(0, 12180))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_976, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Paras FutureCodeUpgrades (r:1 w:0)
	/// Proof Skipped: Paras FutureCodeUpgrades (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras Heads (r:0 w:1)
	/// Proof Skipped: Paras Heads (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeGoAheadSignal (r:0 w:1)
	/// Proof Skipped: Paras UpgradeGoAheadSignal (max_values: None, max_size: None, mode: Measured)
	/// The range of component `s` is `[1, 1048576]`.
	fn force_note_new_head(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `95`
		//  Estimated: `3560`
		// Minimum execution time: 14_408_000 picoseconds.
		Weight::from_parts(14_647_000, 0)
			.saturating_add(Weight::from_parts(0, 3560))
			// Standard Error: 2
			.saturating_add(Weight::from_parts(858, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	fn force_queue_action() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4288`
		//  Estimated: `7753`
		// Minimum execution time: 20_009_000 picoseconds.
		Weight::from_parts(20_518_000, 0)
			.saturating_add(Weight::from_parts(0, 7753))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteList (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 3145728]`.
	fn add_trusted_validation_code(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `946`
		//  Estimated: `4411`
		// Minimum execution time: 80_626_000 picoseconds.
		Weight::from_parts(52_721_755, 0)
			.saturating_add(Weight::from_parts(0, 4411))
			// Standard Error: 1
			.saturating_add(Weight::from_parts(1_443, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Paras CodeByHashRefs (r:1 w:0)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:0 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	fn poke_unused_validation_code() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `28`
		//  Estimated: `3493`
		// Minimum execution time: 6_692_000 picoseconds.
		Weight::from_parts(7_009_000, 0)
			.saturating_add(Weight::from_parts(0, 3493))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	fn include_pvf_check_statement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `26682`
		//  Estimated: `30147`
		// Minimum execution time: 87_994_000 picoseconds.
		Weight::from_parts(89_933_000, 0)
			.saturating_add(Weight::from_parts(0, 30147))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteList (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras UpcomingUpgrades (r:1 w:1)
	/// Proof Skipped: Paras UpcomingUpgrades (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeUpgrades (r:0 w:100)
	/// Proof Skipped: Paras FutureCodeUpgrades (max_values: None, max_size: None, mode: Measured)
	fn include_pvf_check_statement_finalize_upgrade_accept() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `27523`
		//  Estimated: `30988`
		// Minimum execution time: 783_222_000 picoseconds.
		Weight::from_parts(794_959_000, 0)
			.saturating_add(Weight::from_parts(0, 30988))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(104))
	}
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	fn include_pvf_check_statement_finalize_upgrade_reject() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `27214`
		//  Estimated: `30679`
		// Minimum execution time: 87_424_000 picoseconds.
		Weight::from_parts(88_737_000, 0)
			.saturating_add(Weight::from_parts(0, 30679))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteList (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteList (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Configuration ActiveConfig (r:1 w:0)
	/// Proof Skipped: Configuration ActiveConfig (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	fn include_pvf_check_statement_finalize_onboarding_accept() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `26991`
		//  Estimated: `30456`
		// Minimum execution time: 612_485_000 picoseconds.
		Weight::from_parts(621_670_000, 0)
			.saturating_add(Weight::from_parts(0, 30456))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: ParasShared ActiveValidatorKeys (r:1 w:0)
	/// Proof Skipped: ParasShared ActiveValidatorKeys (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:1)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	fn include_pvf_check_statement_finalize_onboarding_reject() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `26682`
		//  Estimated: `30147`
		// Minimum execution time: 86_673_000 picoseconds.
		Weight::from_parts(87_424_000, 0)
			.saturating_add(Weight::from_parts(0, 30147))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}