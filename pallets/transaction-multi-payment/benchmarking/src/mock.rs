#![cfg(test)]

use crate::Trait;
use frame_support::{impl_outer_dispatch, impl_outer_origin, parameter_types, weights::Weight};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
	Perbill,
};

use frame_support::weights::IdentityFee;
use orml_currencies::BasicCurrencyAdapter;
use pallet_transaction_multi_payment::MultiCurrencyAdapter;
use primitives::{Amount, AssetId, Balance};

use frame_support::traits::Get;
use std::cell::RefCell;
use pallet_amm::AssetPairAccountIdFor;

pub type AccountId = u64;

pub const SUPPORTED_CURRENCY: AssetId = 3;

thread_local! {
		static EXTRINSIC_BASE_WEIGHT: RefCell<u64> = RefCell::new(0);
}

pub struct ExtrinsicBaseWeight;
impl Get<u64> for ExtrinsicBaseWeight {
	fn get() -> u64 {
		EXTRINSIC_BASE_WEIGHT.with(|v| *v.borrow())
	}
}

impl_outer_origin! {
	pub enum Origin for Test {}
}

mod multi_payment {
	pub use super::super::*;
}

impl_outer_dispatch! {
	pub enum Call for Test where origin: Origin {
		pallet_balances::Balances,
		frame_system::System,
	}
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;
parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: Weight = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);

	pub const HdxAssetId: u32 = 0;
	pub const ExistentialDeposit: u128 = 0;
	pub const MaxLocks: u32 = 50;
	pub const TransactionByteFee: Balance = 1;
	pub NonNativeAssets: Vec<AssetId> = vec![SUPPORTED_CURRENCY];
}

impl system::Trait for Test {
	type BaseCallFilter = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = ();
	type BlockHashCount = BlockHashCount;
	type MaximumBlockWeight = MaximumBlockWeight;
	type DbWeight = ();
	type BlockExecutionWeight = ();
	type ExtrinsicBaseWeight = ExtrinsicBaseWeight;
	type MaximumExtrinsicWeight = MaximumBlockWeight;
	type MaximumBlockLength = MaximumBlockLength;
	type AvailableBlockRatio = AvailableBlockRatio;
	type Version = ();
	type PalletInfo = ();
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
}
impl Trait for Test {}

impl pallet_transaction_multi_payment::Trait for Test {
	type Event = ();
	type Currency = Balances;
	type MultiCurrency = Currencies;
	type AMMPool = AMMModule;
	type NonNativeAcceptedAssetId = NonNativeAssets;
	type WeightInfo = ();
}

impl pallet_asset_registry::Trait for Test {
	type AssetId = AssetId;
}

impl pallet_balances::Trait for Test {
	type MaxLocks = MaxLocks;
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = ();
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl pallet_transaction_payment::Trait for Test {
	type OnChargeTransaction = MultiCurrencyAdapter<Balances, (), PaymentModule>;
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = ();
}

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

impl pallet_amm::Trait for Test {
	type Event = ();
	type AssetPairAccountId = AssetPairAccountIdTest;
	type Currency = Currencies;
	type HDXAssetId = HdxAssetId;
	type WeightInfo = ();
}

impl orml_tokens::Trait for Test {
	type Event = ();
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	type OnReceived = ();
	type WeightInfo = ();
}

impl orml_currencies::Trait for Test {
	type Event = ();
	type MultiCurrency = Tokens;
	type NativeCurrency = BasicCurrencyAdapter<Test, Balances, Amount, u32>;
	type GetNativeCurrencyId = HdxAssetId;
	type WeightInfo = ();
}

pub type AMMModule= pallet_amm::Module<Test>;
pub type Tokens = orml_tokens::Module<Test>;
pub type Currencies = orml_currencies::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;

pub type PaymentModule = pallet_transaction_multi_payment::Module<Test>;
pub type System = system::Module<Test>;

pub struct ExtBuilder {
	base_weight: u64,
	native_balances: Vec<(AccountId, Balance)>,
	endowed_accounts: Vec<(AccountId, AssetId, Balance)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			base_weight: 0,
			native_balances: vec![(1,100_000)],
			endowed_accounts: vec![],
		}
	}
}

impl ExtBuilder {
	pub fn base_weight(mut self, base_weight: u64) -> Self {
		self.base_weight = base_weight;
		self
	}
	fn set_constants(&self) {
		EXTRINSIC_BASE_WEIGHT.with(|v| *v.borrow_mut() = self.base_weight);
	}
	pub fn build(self) -> sp_io::TestExternalities {
		self.set_constants();
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		pallet_balances::GenesisConfig::<Test> {
			balances: self.native_balances,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		orml_tokens::GenesisConfig::<Test> {
			endowed_accounts: self.endowed_accounts,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let core_asset: u32 = 0;
		let mut buf: Vec<u8> = Vec::new();

		buf.extend_from_slice(&core_asset.to_le_bytes());
		buf.extend_from_slice(b"HDT");
		buf.extend_from_slice(&core_asset.to_le_bytes());

		pallet_asset_registry::GenesisConfig::<Test> {
			core_asset_id: 0,
			next_asset_id: 2,
			asset_ids: vec![(buf.to_vec(), 1)],
		}
		.assimilate_storage(&mut t)
		.unwrap();

		t.into()
	}
}
