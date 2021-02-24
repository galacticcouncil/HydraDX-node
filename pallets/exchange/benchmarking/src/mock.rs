#![cfg(test)]

use super::*;
use frame_support::{impl_outer_origin, parameter_types};
use frame_system as system;
use orml_traits::parameter_type_with_key;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup, Zero},
};

use pallet_amm::AssetPairAccountIdFor;
use primitives::{fee, AssetId, Balance};

pub type Amount = i128;
pub type AccountId = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const CHARLIE: AccountId = 3;
pub const DAVE: AccountId = 4;
pub const FERDIE: AccountId = 5;
pub const GEORGE: AccountId = 6;

pub const HDX: AssetId = 1000;
pub const DOT: AssetId = 2000;
pub const ETH: AssetId = 3000;

impl_outer_origin! {
	pub enum Origin for Test where system = frame_system {}
}

// For testing the pallet, we construct most of a mock runtime. This means
// first constructing a configuration type (`Test`) which `impl`s each of the
// configuration traits of pallets we want to use.
#[derive(Clone, Eq, PartialEq)]
pub struct Test;

parameter_types! {
	pub const BlockHashCount: u64 = 250;

	pub const HDXAssetId: AssetId = HDX;

	pub ExchangeFeeRate: fee::Fee = fee::Fee::default();
}
impl system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type Origin = Origin;
	type Call = ();
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = ();
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}

parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: AssetId| -> Balance {
		Zero::zero()
	};
}

impl orml_tokens::Config for Test {
	type Event = ();
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type WeightInfo = ();
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = ();
}

pub type Currency = orml_tokens::Module<Test>;

pub struct AssetPairAccountIdTest();

impl AssetPairAccountIdFor<AssetId, u64> for AssetPairAccountIdTest {
	fn from_assets(asset_a: AssetId, asset_b: AssetId) -> u64 {
		let mut a = asset_a as u128;
		let mut b = asset_b as u128;
		if a > b {
			let tmp = a;
			a = b;
			b = tmp;
		}
		return (a * 1000 + b) as u64;
	}
}

impl pallet_asset_registry::Config for Test {
	type AssetId = AssetId;
}

impl pallet_amm::Config for Test {
	type Event = ();
	type AssetPairAccountId = AssetPairAccountIdTest;
	type Currency = Currency;
	type HDXAssetId = HDXAssetId;
	type WeightInfo = ();
	type GetExchangeFee = ExchangeFeeRate;
}

pub type AMMModule = pallet_amm::Module<Test>;
pub type System = system::Module<Test>;

impl pallet_exchange::Config for Test {
	type Event = ();
	type AMMPool = AMMModule;
	type Currency = Currency;
	type Resolver = pallet_exchange::Module<Test>;
	type WeightInfo = ();
}

pub struct ExtBuilder {
	endowed_accounts: Vec<(AccountId, AssetId, Balance)>,
}

impl crate::Config for Test {}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			endowed_accounts: vec![
				(ALICE, HDX, 1000_000_000_000_000u128),
				(BOB, HDX, 1000_000_000_000_000u128),
				(CHARLIE, HDX, 1000_000_000_000_000u128),
				(DAVE, HDX, 1000_000_000_000_000u128),
				(FERDIE, HDX, 1000_000_000_000_000u128),
				(GEORGE, HDX, 1000_000_000_000_000u128),
				(ALICE, ETH, 1000_000_000_000_000u128),
				(BOB, ETH, 1000_000_000_000_000u128),
				(CHARLIE, ETH, 1000_000_000_000_000u128),
				(DAVE, ETH, 1000_000_000_000_000u128),
				(FERDIE, ETH, 1000_000_000_000_000u128),
				(GEORGE, ETH, 1000_000_000_000_000u128),
				(ALICE, DOT, 1000_000_000_000_000u128),
				(BOB, DOT, 1000_000_000_000_000u128),
				(CHARLIE, DOT, 1000_000_000_000_000u128),
				(DAVE, DOT, 1000_000_000_000_000u128),
				(FERDIE, DOT, 1000_000_000_000_000u128),
				(GEORGE, DOT, 1000_000_000_000_000u128),
			],
		}
	}
}

impl ExtBuilder {
	// builds genesis config

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		orml_tokens::GenesisConfig::<Test> {
			endowed_accounts: self.endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}

pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut ext = ExtBuilder::default().build();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
