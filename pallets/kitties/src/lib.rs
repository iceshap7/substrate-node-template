#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::pallet_prelude::{*, OptionQuery};
use frame_system::pallet_prelude::*;
use scale_info::prelude::vec::Vec;
use frame_support::traits::UnixTime;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[derive(TypeInfo, Encode, Decode)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender {
		fn default() -> Self {
			Gender::Male
		}
	}

	#[derive(TypeInfo, Default, Encode, Decode)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitty<T: Config> {
		dna: Vec<u8>,
		owner: T::AccountId,
		price: u32,
		gender: Gender,
		created_at: u128,
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type TimeProvider: UnixTime;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn total)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Total<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub type KittyMap<T: Config> = StorageMap<_, Blake2_128Concat, Vec<u8>, Kitty<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn kitty_by_owner)]
	pub type KittyByOwnerMap<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		KittyStored(Vec<u8>, T::AccountId),
		KittySwapped(Vec<u8>, T::AccountId, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_kitty(origin: OriginFor<T>, _dna: Vec<u8>, _price: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			let _gender = Self::gender_gen(_dna.clone())?;
			let now = T::TimeProvider::now();
			let kitty = Kitty::<T> {
				dna: _dna.clone(),
				owner: who.clone(),
				price: _price,
				gender: _gender,
				created_at: now.as_millis(),
			};

			let old_kitty_by_owner = <KittyByOwnerMap<T>>::get(who.clone());
			let mut kitty_by_owner = match old_kitty_by_owner {
				Some(kitties) => kitties,
				None => Vec::new(),
			};
			kitty_by_owner.push(_dna.clone());

			let old_total = <Total<T>>::get();
			let mut total = match old_total {
				Some(val) => val,
				None => 0,
			};
			total += 1;

			// Update storage.
			<KittyMap<T>>::insert(_dna.clone(), kitty);
			<KittyByOwnerMap<T>>::insert(who.clone(), kitty_by_owner);
			<Total<T>>::put(total);

			// Emit an event.
			Self::deposit_event(Event::KittyStored(_dna, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn swap_kitty(origin: OriginFor<T>, _dna: Vec<u8>, _to_account: T::AccountId) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			let mut kitty = <KittyMap<T>>::get(_dna.clone()).unwrap();
			assert_eq!(who.clone(), kitty.owner);
			kitty.owner = _to_account.clone();

			let old_kitty_by_signed_account = <KittyByOwnerMap<T>>::get(who.clone());
			let mut kitty_by_signed_account = match old_kitty_by_signed_account {
				Some(kitties) => kitties,
				None => Vec::new(),
			};
			let index = kitty_by_signed_account.iter().position(|x| *x == _dna).unwrap();
			kitty_by_signed_account.remove(index);

			let old_kitty_by_to_account = <KittyByOwnerMap<T>>::get(_to_account.clone());
			let mut kitty_by_to_account = match old_kitty_by_to_account {
				Some(kitties) => kitties,
				None => Vec::new(),
			};
			kitty_by_to_account.push(_dna.clone());

			// Update storage.
			<KittyMap<T>>::insert(_dna.clone(), kitty);
			<KittyByOwnerMap<T>>::insert(who.clone(), kitty_by_signed_account);
			<KittyByOwnerMap<T>>::insert(_to_account.clone(), kitty_by_to_account);

			// Emit an event.
			Self::deposit_event(Event::KittySwapped(_dna, who, _to_account));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}

impl<T> Pallet<T> {
	fn gender_gen(dna: Vec<u8>) -> Result<Gender, Error<T>> {
		let mut res = Gender::Male;
		if dna.len() % 2 != 0 {
			res = Gender::Female;
		}

		Ok(res)
	}
}
