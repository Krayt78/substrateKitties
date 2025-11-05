use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::KittyAlreadyMinted);
		let current_kitty_count = KittyCount::<T>::get();
		let new_kitty_count = current_kitty_count.checked_add(1).ok_or(Error::<T>::KittyCountOverflow)?;
		KittyCount::<T>::set(new_kitty_count);
		Kitties::<T>::insert(dna, Kitty { owner: owner.clone(), dna });
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
