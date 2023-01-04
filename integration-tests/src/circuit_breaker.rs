#![cfg(test)]

use crate::polkadot_test_net::*;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use hydradx_runtime::{Balances, CircuitBreaker, Omnipool, Tokens};
use orml_traits::MultiCurrency;
use primitives::constants::chain::CORE_ASSET_ID;
use sp_runtime::FixedU128;
use sp_runtime::Permill;
use xcm_emulator::TestExt;

#[test]
fn sell_in_omnipool_should_work_when_max_trade_limit_per_block_not_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		assert_ok!(Omnipool::initialize_pool(
			RawOrigin::Root.into(),
			FixedU128::from_float(0.00001), // adjust the amount of LRNA to roughly match the amount of LRNA that belongs to HDX. This way we can avoid MaxOutRatioExceeded error.
			FixedU128::from(1),
			Permill::from_percent(100),
			Permill::from_percent(100)
		));

		let dai_balance_in_omnipool = Tokens::free_balance(DAI, &Omnipool::protocol_account());
		let trade_volume_limit = CircuitBreaker::trade_volume_limit_per_asset(DAI);
		let sell_amount = CircuitBreaker::calculate_limit(dai_balance_in_omnipool, trade_volume_limit).unwrap();

		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			DAI,
			sell_amount,
			0,
		));

		let min_limit = 0;

		//Act and assert
		assert_ok!(Omnipool::sell(
			hydradx_runtime::Origin::signed(ALICE.into()),
			DAI,
			CORE_ASSET_ID,
			sell_amount,
			min_limit
		));
	});
}

#[test]
fn sell_in_omnipool_should_fail_when_max_trade_limit_per_block_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		assert_ok!(Omnipool::initialize_pool(
			RawOrigin::Root.into(),
			FixedU128::from_float(0.00001), // adjust the amount of LRNA to roughly match the amount of LRNA that belongs to HDX. This way we can avoid MaxOutRatioExceeded error.
			FixedU128::from(1),
			Permill::from_percent(100),
			Permill::from_percent(100)
		));

		let dai_balance_in_omnipool = Tokens::free_balance(DAI, &Omnipool::protocol_account());
		let trade_volume_limit = CircuitBreaker::trade_volume_limit_per_asset(DAI);
		let sell_amount = CircuitBreaker::calculate_limit(dai_balance_in_omnipool, trade_volume_limit).unwrap().checked_add(1).unwrap();

		assert_ok!(Tokens::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			DAI,
			sell_amount,
			0,
		));

		let min_limit = 0;

		//Act and assert
		assert_noop!(
			Omnipool::sell(
				hydradx_runtime::Origin::signed(ALICE.into()),
				DAI,
				CORE_ASSET_ID,
				sell_amount,
				min_limit
			),
			pallet_circuit_breaker::Error::<hydradx_runtime::Runtime>::MaxTradeVolumePerBlockReached
		);
	});
}

#[test]
fn add_liquidity_to_omnipool_should_work_when_liquidity_limit_per_block_not_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		assert_ok!(Omnipool::initialize_pool(
			RawOrigin::Root.into(),
			FixedU128::from_float(0.65),
			FixedU128::from(1),
			Permill::from_percent(100),
			Permill::from_percent(100)
		));

		let hdx_balance_in_omnipool = Balances::free_balance(&Omnipool::protocol_account());
		let liquidity_limit = CircuitBreaker::liquidity_limit_per_asset(CORE_ASSET_ID).unwrap();
		let added_liquidity = CircuitBreaker::calculate_limit(hdx_balance_in_omnipool, liquidity_limit).unwrap();

		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			added_liquidity,
			0,
		));

		//Act and assert
		assert_ok!(Omnipool::add_liquidity(
			hydradx_runtime::Origin::signed(ALICE.into()),
			CORE_ASSET_ID,
			added_liquidity,
		));
	});
}

#[test]
fn add_liquidity_to_omnipool_should_fail_when_liquidity_limit_per_block_exceeded() {
	Hydra::execute_with(|| {
		//Arrange
		assert_ok!(Omnipool::initialize_pool(
			RawOrigin::Root.into(),
			FixedU128::from_float(0.65),
			FixedU128::from(1),
			Permill::from_percent(100),
			Permill::from_percent(100)
		));

		let hdx_balance_in_omnipool = Balances::free_balance(&Omnipool::protocol_account());
		let liquidity_limit = CircuitBreaker::liquidity_limit_per_asset(CORE_ASSET_ID).unwrap();
		let added_liquidity = CircuitBreaker::calculate_limit(hdx_balance_in_omnipool, liquidity_limit).unwrap().checked_add(1).unwrap();

		assert_ok!(Balances::set_balance(
			RawOrigin::Root.into(),
			ALICE.into(),
			added_liquidity,
			0,
		));

		//Act and assert
		assert_noop!(Omnipool::add_liquidity(
			hydradx_runtime::Origin::signed(ALICE.into()),
			CORE_ASSET_ID,
			added_liquidity,
		),
			pallet_circuit_breaker::Error::<hydradx_runtime::Runtime>::MaxLiquidityLimitPerBlockReached
		);
	});
}