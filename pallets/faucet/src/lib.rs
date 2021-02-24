#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
	weights::{DispatchClass, Pays},
};
use frame_system::ensure_signed;
use orml_traits::{MultiCurrency, MultiCurrencyExtended};
use primitives::{AssetId, Balance};
use sp_std::vec::Vec;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub trait Config: frame_system::Config {
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
	type Currency: MultiCurrencyExtended<Self::AccountId, CurrencyId = AssetId, Balance = Balance, Amount = i128>;
}

decl_storage! {
	trait Store for Module<T: Config> as Faucet {
		pub Minted get(fn minted): u8;
		pub MintLimit get(fn mint_limit) config(): u8;
		pub Rampage get(fn rampage) config(): bool;
		pub MintableCurrencies get(fn mintable_currencies) config(): Vec<AssetId>;
	}
}

decl_event!(
	pub enum Event<T>
	where
		AccountId = <T as frame_system::Config>::AccountId,
		AssetId = AssetId,
		Balance = Balance,
	{
		RampageMint(AccountId, AssetId, Balance),
		Mint(AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		RampageMintNotAllowed,
		MaximumMintLimitReached
	}
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		#[weight = (0, DispatchClass::Normal, Pays::No)]
		pub fn rampage_mint(origin, asset: AssetId, amount: Balance) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(
				Self::rampage(),
				Error::<T>::RampageMintNotAllowed
			);

			T::Currency::deposit(asset, &who, amount)?;
			Self::deposit_event(RawEvent::RampageMint(who, asset, amount));

			Ok(())
		}

		#[weight = (0, DispatchClass::Normal, Pays::No)]
		pub fn mint(origin) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(Self::minted() < Self::mint_limit(), Error::<T>::MaximumMintLimitReached);

			for i in Self::mintable_currencies() {
				T::Currency::deposit(i, &who, 1_000_000_000_000_000)?;
			}

			Minted::set(Self::minted() + 1);

			Self::deposit_event(RawEvent::Mint(who));

			Ok(())
		}

		fn on_finalize(){
			Minted::set(0u8);
		}
	}
}
