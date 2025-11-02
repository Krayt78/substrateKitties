use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		let current_kitty_count = KittyCount::<T>::get();
		let new_kitty_count = current_kitty_count.checked_add(1).ok_or(Error::<T>::KittyCountOverflow)?;
		KittyCount::<T>::set(new_kitty_count);
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
