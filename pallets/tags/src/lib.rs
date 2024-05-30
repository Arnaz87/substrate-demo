//! # Tags Pallet
//!
//! This pallet implements a basic tag system, where accounts reserve some amount of their funds
//! for the creation of a tag, that has an associated name.
//!
//! This pallet is intended to be used along with nfts, but the support is not yet implemented.

// We make sure this pallet uses `no_std` for compiling to Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

// FRAME pallets require their own "mock runtimes" to be able to run unit tests. This module
// contains a mock runtime specific for testing this pallet's functionality.
#[cfg(test)]
mod mock;

// This module contains the unit tests for this pallet.
// Learn about pallet unit testing here: https://docs.substrate.io/test/unit-testing/
#[cfg(test)]
mod tests;

// Every callable function or "dispatchable" a pallet exposes must have weight values that correctly
// estimate a dispatchable's execution time. The benchmarking module is used to calculate weights
// for each dispatchable and generates this pallet's weight.rs file. Learn more about benchmarking here: https://docs.substrate.io/test/benchmark/
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

// All pallet logic is defined in its own module and must be annotated by the `pallet` attribute.
#[frame_support::pallet]
pub mod pallet {
	// Import various useful types required by all FRAME pallets.
	use super::*;
	use frame_support::{pallet_prelude::*, traits::{Currency, ReservableCurrency}};
	use frame_system::pallet_prelude::*;

	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	// The `Pallet` struct serves as a placeholder to implement traits, methods and dispatchables
	// (`Call`s) in this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// The pallet's configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching runtime event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// A type representing the weights required by the dispatchables of this pallet.
		type WeightInfo: WeightInfo;
		/// The currency trait
		type Currency: ReservableCurrency<Self::AccountId>;

		/// The maximum length of data stored on-chain.
		#[pallet::constant]
		type TagNameLimit: Get<u32>;

		/// The deposit necessary to create a tag
		#[pallet::constant]
		type TagDepositAmount: Get<BalanceOf<Self>>;
	}

	/// Counter of the next available index for a tag
	#[pallet::storage]
	pub type TagIndex<T> = StorageValue<_, u64, ValueQuery>;

	/// Tags stored in the network
	#[pallet::storage]
	#[pallet::getter(fn tag_info)]
	pub type TagMap<T: Config> = StorageMap<
		Hasher = Blake2_128Concat,
		Key = u64,
		Value = (
			BoundedVec<u8, T::TagNameLimit>, // name
			T::AccountId, // creator
			BalanceOf<T>, // deposit
		),
		QueryKind = OptionQuery
	>;


	/// Events that functions in this pallet can emit.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A user has successfully set a new value.
		TagCreated {
			/// The tag id.
			index: u64,
			/// The account who created the tag.
			who: T::AccountId,
			/// The deposit reserved for the tag.
			deposit: BalanceOf<T>,
		},
		/// A user destroyed a previously created tag.
		TagDestroyed {
			/// The tag id.
			index: u64,
			/// The account that owned and destroyed the tag
			who: T::AccountId,
		},
	}

	/// Errors that can be returned by this pallet.
	#[pallet::error]
	pub enum Error<T> {
		/// The value retrieved was `None` as no value was previously set.
		NoneValue,
		/// There are more tags than possible to store on chain (u64 id limit).
		StorageOverflow,
		/// The tag with the given ID doesn't exist in the network.
		InvalidTag,
		/// An user tried to modify a tag that it didn't create.
		NotAllowed,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Creates a tag with a name.
		/// Tags are stored in TagMap, they contain a name, the creator, and the deposit reserved for them.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_tag())]
		pub fn create_tag(origin: OriginFor<T>, name: BoundedVec<u8, T::TagNameLimit>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// Amount to deposit. Comes from configuration but it's good practice to store any amount
			// reserved at any point.
			let deposit = T::TagDepositAmount::get();

			// Try reserving the amount. This function naturally fails if the account lacks funds.
			T::Currency::reserve(&who, deposit)?;

			// Get the next available index and update the counter
			let index = TagIndex::<T>::get();

			// Increment the tag index. This will cause an error in the event
			// of overflow.
			TagIndex::<T>::put(
				index.checked_add(1).ok_or(Error::<T>::StorageOverflow)?
			);

			TagMap::<T>::insert(index, (name, who.clone(), deposit));

			// Emit the corresponding event.
			Self::deposit_event(Event::TagCreated {
				index, who, deposit
			});

			// Return a successful `DispatchResult`
			Ok(())
		}

		/// Destroys a tag from the chain
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::destroy_tag())]
		pub fn destroy_tag(origin: OriginFor<T>, tag_index: u64) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			let (_name, creator, deposit) = TagMap::<T>::try_get(tag_index).map_err(|()|Error::<T>::InvalidTag)?;

			if who != creator {
				Err(Error::<T>::NotAllowed)?;
			}

			// Unreserve doesn't fail, unlike reserve
			T::Currency::unreserve(&who, deposit);

			TagMap::<T>::remove(tag_index);

			// Emit the corresponding event.
			Self::deposit_event(Event::TagDestroyed { index: tag_index, who });

			Ok(())
		}
	}
}
