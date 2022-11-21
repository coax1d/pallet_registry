#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_support::traits::{ReservableCurrency, LockableCurrency};
use frame_system::pallet_prelude::*;
	#[cfg(feature = "std")]
	use serde::{Deserialize, Serialize};
	use scale_info::TypeInfo;

	// const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Max Length in bytes for for a Persons name
		#[pallet::constant]
		type MaxNameLengthBytes: Get<u32>;

		/// Max Age of Person that can be stored in registry
		#[pallet::constant]
		type MaxAgeOfPerson: Get<u32>;

		type MyCurrency: ReservableCurrency<Self::AccountId> + LockableCurrency<Self::AccountId>;
	}

	pub type Name<T> = BoundedVec<u8, <T as Config>::MaxNameLengthBytes>;
	pub type Age = u32;

	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct Person<T: Config> {
		pub name: Name<T>,
		pub age: Age,
		pub job: Job,
	}

	#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
	#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Encode, Decode, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub enum Job {
		Programmer,
		Artist,
	}

	#[pallet::storage]
	#[pallet::getter(fn people_registry)]
	pub type PeopleRegistry<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Person<T>,
		OptionQuery
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		PersonAdded {
			account_id: T::AccountId,
			age: Age,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Trying to add a Person over the age limit
		MaxAgeExceeded,
		/// Person already exists in the Registry
		AlreadyExistsInRegistry,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn add_registry(
			origin: OriginFor<T>,
			name: Name<T>,
			age: Age,
			job: Job,
		) -> DispatchResult {
			let signer = ensure_signed(origin)?;

			ensure!(
				!<PeopleRegistry<T>>::contains_key(&signer),
				Error::<T>::AlreadyExistsInRegistry
			);
			ensure!(
				age < T::MaxAgeOfPerson::get(),
				Error::<T>::MaxAgeExceeded
			);

			let _signers_reserve_balance =
				T::MyCurrency::reserved_balance(&signer);
				// <<T as Config>::MyCurrency as ReservableCurrency<T::AccountId>>::reserved_balance(&signer);

			let person_to_store = Person::<T> {
				name,
				age,
				job,
			};
			<PeopleRegistry<T>>::insert(&signer, person_to_store);
			Self::deposit_event(Event::<T>::PersonAdded { account_id: signer, age: age });
			Ok(())
		}
	}
}
