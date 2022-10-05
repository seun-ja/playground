#![cfg_attr(not(feature = "std"), no_std)]
#![feature(option_result_contains)]

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

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::{*, DispatchResult};
	use frame_system::pallet_prelude::*;
	use frame_support::inherent::Vec;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::storage]
	#[pallet::getter(fn club_members)]
	pub(super) type ClubMembers<T: Config> = StorageMap<_, Blake2_128Concat, u32, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn members)]
	pub(super) type Members<T: Config> = StorageValue<_, Vec<T::AccountId>>; //members

	#[derive(Encode, Decode, Clone, RuntimeDebug)]
	pub struct Club<T> {
		pub id: u32,
		members: Vec<T>,
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Add member to the Club
		AddMember(T::AccountId),
		/// Remove member to the Club
		RemoveMember(T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		///Member already in the club
		MemnerAlreadyExists,
		/// Member not valid
		MemberInvalid,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn add_member(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed_or_root(origin).unwrap().unwrap();

			<Members<T>>::append(who.clone());

			Self::deposit_event(Event::AddMember(who));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn remove_member(origin: OriginFor<T>, club_id: u32) -> DispatchResult {
			let who = ensure_signed_or_root(origin).unwrap().unwrap();

			if <ClubMembers<T>>::get(club_id).contains(&who.clone()) {
				<Members<T>>::take();
			};			

			Self::deposit_event(Event::RemoveMember(who));

			Ok(())
		}
	}
}
