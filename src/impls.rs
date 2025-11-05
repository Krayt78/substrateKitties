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
		Kitties::<T>::insert(dna, Kitty { owner: owner.clone(), dna });
		KittiesOwned::<T>::try_append(&owner, Kitty { owner: owner.clone(), dna }).map_err(|_| Error::<T>::TooManyKittiesOwned)?;
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
}
