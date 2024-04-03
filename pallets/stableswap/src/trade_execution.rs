use crate::types::AssetAmount;
use crate::{Balance, Config, Error, Pallet, Pools, D_ITERATIONS, Y_ITERATIONS};
use frame_support::storage::with_transaction;
use frame_system::pallet_prelude::OriginFor;
use frame_system::Origin;
use hydradx_traits::router::{ExecutorError, PoolType, TradeExecution};
use orml_traits::MultiCurrency;
use sp_core::Get;
use sp_runtime::DispatchError::Corruption;
use sp_runtime::{ArithmeticError, DispatchError, FixedU128, TransactionOutcome};
use sp_std::vec;

impl<T: Config> TradeExecution<T::RuntimeOrigin, T::AccountId, T::AssetId, Balance> for Pallet<T> {
	type Error = DispatchError;

	fn calculate_sell(
		pool_type: PoolType<T::AssetId>,
		asset_in: T::AssetId,
		asset_out: T::AssetId,
		amount_in: Balance,
	) -> Result<Balance, ExecutorError<Self::Error>> {
		match pool_type {
			PoolType::Stableswap(pool_id) => {
				if asset_in == pool_id {
					let pool = Pools::<T>::get(pool_id)
						.ok_or_else(|| ExecutorError::Error(Error::<T>::PoolNotFound.into()))?;
					let asset_idx = pool
						.find_asset(asset_out)
						.ok_or_else(|| ExecutorError::Error(Error::<T>::AssetNotInPool.into()))?;
					let pool_account = Self::pool_account(pool_id);
					let balances = pool
						.reserves_with_decimals::<T>(&pool_account)
						.ok_or_else(|| ExecutorError::Error(Error::<T>::UnknownDecimals.into()))?;
					let share_issuance = T::Currency::total_issuance(pool_id);

					let amplification = Self::get_amplification(&pool);
					let (amount, _) = hydra_dx_math::stableswap::calculate_withdraw_one_asset::<
						D_ITERATIONS,
						Y_ITERATIONS,
					>(&balances, amount_in, asset_idx, share_issuance, amplification, pool.fee)
					.ok_or_else(|| ExecutorError::Error(ArithmeticError::Overflow.into()))?;

					Ok(amount)
				} else if asset_out == pool_id {
					let share_amount = Self::calculate_shares(
						pool_id,
						&[AssetAmount {
							asset_id: asset_in,
							amount: amount_in,
						}],
					)
					.map_err(ExecutorError::Error)?;

					Ok(share_amount)
				} else {
					let (amount_out, _) = Self::calculate_out_amount(pool_id, asset_in, asset_out, amount_in)
						.map_err(ExecutorError::Error)?;

					Ok(amount_out)
				}
			}
			_ => Err(ExecutorError::NotSupported),
		}
	}

	fn calculate_buy(
		pool_type: PoolType<T::AssetId>,
		asset_in: T::AssetId,
		asset_out: T::AssetId,
		amount_out: Balance,
	) -> Result<Balance, ExecutorError<Self::Error>> {
		match pool_type {
			PoolType::Stableswap(pool_id) => {
				if asset_out == pool_id {
					//I wanna buy 500 shares, how much luqidity i need provide to get 500 shares
					let pool = Pools::<T>::get(pool_id)
						.ok_or_else(|| ExecutorError::Error(Error::<T>::PoolNotFound.into()))?;
					let asset_idx = pool
						.find_asset(asset_in)
						.ok_or_else(|| ExecutorError::Error(Error::<T>::AssetNotInPool.into()))?;
					let pool_account = Self::pool_account(pool_id);
					let balances = pool
						.reserves_with_decimals::<T>(&pool_account)
						.ok_or_else(|| ExecutorError::Error(Error::<T>::UnknownDecimals.into()))?;
					let share_issuance = T::Currency::total_issuance(pool_id);
					let amplification = Self::get_amplification(&pool);

					let liqudity = hydra_dx_math::stableswap::calculate_add_one_asset::<D_ITERATIONS, Y_ITERATIONS>(
						&balances,
						amount_out,
						asset_idx,
						share_issuance,
						amplification,
						pool.fee,
					)
					.ok_or_else(|| ExecutorError::Error(ArithmeticError::Overflow.into()))?;

					Ok(liqudity.0)
				} else if asset_in == pool_id {
					let pool = Pools::<T>::get(pool_id)
						.ok_or_else(|| ExecutorError::Error(Error::<T>::PoolNotFound.into()))?;
					let asset_idx = pool
						.find_asset(asset_out)
						.ok_or_else(|| ExecutorError::Error(Error::<T>::AssetNotInPool.into()))?;
					let pool_account = Self::pool_account(pool_id);
					let balances = pool
						.reserves_with_decimals::<T>(&pool_account)
						.ok_or_else(|| ExecutorError::Error(Error::<T>::UnknownDecimals.into()))?;
					let share_issuance = T::Currency::total_issuance(pool_id);
					let amplification = Self::get_amplification(&pool);

					let pool = Pools::<T>::get(pool_id)
						.ok_or_else(|| ExecutorError::Error(Error::<T>::PoolNotFound.into()))?;

					let shares_amount = hydra_dx_math::stableswap::calculate_shares_for_amount::<D_ITERATIONS>(
						&balances,
						asset_idx,
						amount_out,
						amplification,
						share_issuance,
						pool.fee,
					)
					.ok_or_else(|| ExecutorError::Error(ArithmeticError::Overflow.into()))?;

					Ok(shares_amount)
				} else {
					let (amount_in, _) = Self::calculate_in_amount(pool_id, asset_in, asset_out, amount_out)
						.map_err(ExecutorError::Error)?;

					Ok(amount_in)
				}
			}
			_ => Err(ExecutorError::NotSupported),
		}
	}

	fn execute_sell(
		who: T::RuntimeOrigin,
		pool_type: PoolType<T::AssetId>,
		asset_in: T::AssetId,
		asset_out: T::AssetId,
		amount_in: Balance,
		min_limit: Balance,
	) -> Result<(), ExecutorError<Self::Error>> {
		match pool_type {
			PoolType::Stableswap(pool_id) => {
				if asset_in == pool_id {
					Self::remove_liquidity_one_asset(who, pool_id, asset_out, amount_in, min_limit)
						.map_err(ExecutorError::Error)
				} else if asset_out == pool_id {
					Self::add_liquidity(
						who,
						pool_id,
						vec![AssetAmount {
							asset_id: asset_in,
							amount: amount_in,
						}],
					)
					.map_err(ExecutorError::Error)
				} else {
					Self::sell(who, pool_id, asset_in, asset_out, amount_in, min_limit).map_err(ExecutorError::Error)
				}
			}
			_ => Err(ExecutorError::NotSupported),
		}
	}

	fn execute_buy(
		who: T::RuntimeOrigin,
		pool_type: PoolType<T::AssetId>,
		asset_in: T::AssetId,
		asset_out: T::AssetId,
		amount_out: Balance,
		max_limit: Balance,
	) -> Result<(), ExecutorError<Self::Error>> {
		match pool_type {
			PoolType::Stableswap(pool_id) => {
				if asset_out == pool_id {
					Self::add_liquidity_shares(who, pool_id, amount_out, asset_in, max_limit)
						.map_err(ExecutorError::Error)
				} else if asset_in == pool_id {
					Self::withdraw_asset_amount(who, pool_id, asset_out, amount_out, max_limit)
						.map_err(ExecutorError::Error)
				} else {
					Self::buy(who, pool_id, asset_out, asset_in, amount_out, max_limit).map_err(ExecutorError::Error)
				}
			}
			_ => Err(ExecutorError::NotSupported),
		}
	}

	fn get_liquidity_depth(
		pool_type: PoolType<T::AssetId>,
		asset_a: T::AssetId,
		_asset_b: T::AssetId,
	) -> Result<Balance, ExecutorError<Self::Error>> {
		match pool_type {
			PoolType::Stableswap(pool_id) => {
				let pool_account = Self::pool_account(pool_id);
				Ok(T::Currency::free_balance(asset_a, &pool_account))
			}
			_ => Err(ExecutorError::NotSupported),
		}
	}

	fn calculate_spot_price(
		pool_type: PoolType<T::AssetId>,
		asset_a: T::AssetId,
		asset_b: T::AssetId,
	) -> Result<FixedU128, ExecutorError<Self::Error>> {
		match pool_type {
			PoolType::Stableswap(pool_id) => {
				let spot_price = with_transaction::<_, DispatchError, _>(|| {
					let amount_in = T::MinTradingLimit::get();
					let origin: OriginFor<T> = Origin::<T>::Signed(Self::pallet_account()).into();

					//We mint amount in to dry-run sell
					let _ = T::Currency::deposit(asset_a, &Self::pallet_account(), amount_in);

					//We need to mint some asset_out balance otherwise we can have ED error triggered when amount_out is transferred to account from sell
					let _ = T::Currency::deposit(asset_b, &Self::pallet_account(), amount_in);

					if let Err(err) = Self::execute_sell(
						origin,
						PoolType::Stableswap(pool_id),
						asset_a,
						asset_b,
						amount_in,
						Balance::MIN,
					) {
						return match err {
							ExecutorError::Error(dispatch_err) => TransactionOutcome::Rollback(Err(dispatch_err)),
							_ => TransactionOutcome::Rollback(Err(Corruption)),
						};
					}

					let Some(amount_out) =
						T::Currency::free_balance(asset_b, &Self::pallet_account()).checked_sub(amount_in) else {
						return TransactionOutcome::Rollback(Err(Corruption));
					};

					let spot_price = FixedU128::from_rational(amount_in, amount_out);

					TransactionOutcome::Rollback(Ok(spot_price))
				})
				.map_err(ExecutorError::Error)?;

				Ok(spot_price)
			}
			_ => Err(ExecutorError::NotSupported),
		}
	}
}
