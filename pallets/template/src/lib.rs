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
	pub(super) type ClubMembers<T: Config> = StorageMap<_, Blake2_128Concat, u32, ClubMembership<T>, ValueQuery>;

	pub type ClubMembership<T> = Club<<T as frame_system::Config>::AccountId>;

	pub type Membership<T> = <T as frame_system::Config>::AccountId;

	#[derive(Encode, Decode, Clone, RuntimeDebug, PartialEq, TypeInfo)]
	pub struct Club<AccountId> {
		pub id: u32,
		members: Vec<AccountId>,
	}

	impl<AccountId> Default for Club<AccountId> {
		fn default() -> Self { 
			Self {
				id: 1,
				members: [].into()
			}
		}
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Introduce Club
		Club(u32, T::AccountId),
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
		pub fn create_club(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed_or_root(origin).unwrap().unwrap();

			// should be a random, but hardcoded for now
			let club_id = 1;

			Self::deposit_event(Event::Club(club_id, who));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_member(origin: OriginFor<T>, club_id: u32) -> DispatchResult {
			let who = ensure_signed_or_root(origin).unwrap().unwrap();

			let mut club = <ClubMembers<T>>::get(club_id);

			club.members.push(who.clone());

			Self::deposit_event(Event::AddMember(who));

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn remove_member(origin: OriginFor<T>, club_id: u32) -> DispatchResult {
			let who = ensure_signed_or_root(origin).unwrap().unwrap();

			let mut index = 0;

			if <ClubMembers<T>>::get(club_id).members.contains(&who.clone()) {
				for member in <ClubMembers<T>>::get(club_id).members {
					if who == member {
						<ClubMembers<T>>::get(club_id).members.remove(index);
					}

					index += 1;
				}
			};			

			Self::deposit_event(Event::RemoveMember(who));

			Ok(())
		}
	}
}
