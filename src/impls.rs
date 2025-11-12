use super::*;
use frame::prelude::*;
use frame::primitives::BlakeTwo256;
use frame::traits::Hash;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		let dna = Self::generate_dna();
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::KittyAlreadyMinted);
		let current_kitty_count = KittyCount::<T>::get();
		let new_kitty_count = current_kitty_count.checked_add(1).ok_or(Error::<T>::KittyCountOverflow)?;
		KittyCount::<T>::set(new_kitty_count);
		Kitties::<T>::insert(dna, Kitty { price:None, owner: owner.clone(), dna });
		KittiesOwned::<T>::try_append(&owner, dna).map_err(|_| Error::<T>::TooManyKittiesOwned)?;
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}

	fn generate_dna() -> [u8; 32] {
		let unique_payload = (
			frame_system::Pallet::<T>::parent_hash(),
			frame_system::Pallet::<T>::block_number(),
			frame_system::Pallet::<T>::extrinsic_index(),
			KittyCount::<T>::get()
		);
		BlakeTwo256::hash_of(&unique_payload).into()
	}

	pub fn do_transfer(from: T::AccountId, to: T::AccountId, kitty_id: [u8; 32]) -> DispatchResult {
		ensure!(from != to, Error::<T>::TransferToSelf);
		let mut kitty = Kitties::<T>::get(kitty_id).ok_or(Error::<T>::KittyNotFound)?;
		ensure!(kitty.owner == from, Error::<T>::NotOwner);

		kitty.owner = to.clone();
		kitty.price = None;
		
		let mut to_owned = KittiesOwned::<T>::get(&to);
		to_owned.try_push(kitty_id).map_err(|_| Error::<T>::TooManyKittiesOwned)?;
		let mut from_owned = KittiesOwned::<T>::get(&from);
		if let Some(ind) = from_owned.iter().position(|&id| id == kitty_id) {
			from_owned.swap_remove(ind);
		} else {
			return Err(Error::<T>::KittyNotFound.into())
		}

		Kitties::<T>::insert(kitty_id, kitty);
		KittiesOwned::<T>::insert(&to, to_owned);
		KittiesOwned::<T>::insert(&from, from_owned);
		
		Self::deposit_event(Event::<T>::Transferred{from, to, kitty_id});
		Ok(())
	}
}
