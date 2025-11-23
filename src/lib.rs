#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
mod tests;

use frame::prelude::*;
pub use pallet::*;
use frame::traits::fungible::Inspect;
use frame::traits::fungible::Mutate;

// Allows easy access our Pallet's `Balance` type. Comes from `Fungible` interface.
pub type BalanceOf<T> =
	<<T as Config>::NativeBalance as Inspect<<T as frame_system::Config>::AccountId>>::Balance;

#[frame::pallet(dev_mode)]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Access the balances pallet through the associated type `NativeBalance`.
		/// The `NativeBalance` type must implement `Inspect` and `Mutate`.
		/// Both of these traits are generic over the `AccountId` type.
		type NativeBalance: Inspect<Self::AccountId> + Mutate<Self::AccountId>;
	}

	#[derive(Encode, Decode, MaxEncodedLen, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		pub price: Option<BalanceOf<T>>,
		pub owner: T::AccountId,
		pub dna: [u8; 32],
	}

	#[pallet::storage]
	pub type KittyCount<T: Config> = StorageValue<Value = u32, QueryKind = ValueQuery>;

	#[pallet::storage]
	pub type Kitties<T: Config> = StorageMap<Value = Kitty<T>, Key = [u8; 32]>;
	
	#[pallet::storage]
	pub type KittiesOwned<T: Config> = StorageMap<Value = BoundedVec<[u8; 32], ConstU32<100>>, Key = T::AccountId, QueryKind = ValueQuery>;
	
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Created { owner: T::AccountId },
		Transferred {from: T::AccountId, to: T::AccountId, kitty_id: [u8;32]},
		PriceSet { kitty_id: [u8; 32], new_price: BalanceOf<T> },
	}

	#[pallet::error]
	pub enum Error<T> {
		KittyCountOverflow,
		KittyAlreadyMinted,
		TooManyKittiesOwned,
		KittyNotFound,
		TransferToSelf,
		NotOwner,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		pub fn create_kitty(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::mint(who)?;
			Ok(())
		}

		pub fn transfer_kitty(origin: OriginFor<T>, to: T::AccountId, kitty_id: [u8; 32]) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_transfer(who, to, kitty_id)?;
			Ok(())
		}

		pub fn set_price(origin: OriginFor<T>, kitty_id: [u8; 32], price: BalanceOf<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_set_price(who, kitty_id, price)?;
			Ok(())
		}
	}
}
