//! # Template Pallet
//!
//! A pallet with minimal functionality to help developers understand the essential components of
//! writing a FRAME pallet. It is typically used in beginner tutorials or in Substrate template
//! nodes as a starting point for creating a new pallet and **not meant to be used in production**.
//!
//! ## Overview
//!
//! This template pallet contains basic examples of:
//! - declaring a storage item that stores a single `u32` value
//! - declaring and using events
//! - declaring and using errors
//! - a dispatchable function that allows a user to set a new value to storage and emits an event
//!   upon success
//! - another dispatchable function that causes a custom error to be thrown
//!
//! Each pallet section is annotated with an attribute using the `#[pallet::...]` procedural macro.
//! This macro generates the necessary code for a pallet to be aggregated into a FRAME runtime.
//!
//! Learn more about FRAME macros [here](https://docs.substrate.io/reference/frame-macros/).
//!
//! ### Pallet Sections
//!
//! The pallet sections in this template are:
//!
//! - A **configuration trait** that defines the types and parameters which the pallet depends on
//!   (denoted by the `#[pallet::config]` attribute). See: [`Config`].
//! - A **means to store pallet-specific data** (denoted by the `#[pallet::storage]` attribute).
//!   See: [`storage_types`].
//! - A **declaration of the events** this pallet emits (denoted by the `#[pallet::event]`
//!   attribute). See: [`Event`].
//! - A **declaration of the errors** that this pallet can throw (denoted by the `#[pallet::error]`
//!   attribute). See: [`Error`].
//! - A **set of dispatchable functions** that define the pallet's functionality (denoted by the
//!   `#[pallet::call]` attribute). See: [`dispatchables`].
//!
//! Run `cargo doc --package pallet-template --open` to view this pallet's documentation.

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
	///
	/// All our types and constants a pallet depends on must be declared here.
	/// These types are defined generically and made concrete when the pallet is declared in the
	/// `runtime/src/lib.rs` file of your chain.
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

	/// A storage item for this pallet.
	///
	/// In this template, we are declaring a storage item called `Something` that stores a single
	/// `u32` value. Learn more about runtime storage here: <https://docs.substrate.io/build/runtime-storage/>
	#[pallet::storage]
	pub type TagIndex<T> = StorageValue<_, u64>;

	/// Tags stored in the network
	#[pallet::storage]
	#[pallet::getter(fn tag_info)]
	pub type TagMap<T: Config> = StorageMap<
		Hasher = Blake2_128Concat,
		Key = u64,
		Value = (
			BoundedVec<u8, T::TagNameLimit>, // name

			// ??? AccountId doesn't implement Default, and it cannot used for storage...
			// I wrap it around an Option so that I can say it has a default.
			// idk why Default is needed for this, and idk why other examples just use T::AccountId and it works for them
			Option<T::AccountId>, // creator
			BalanceOf<T>, // deposit
		),
		QueryKind = ValueQuery
	>;


	/// Events that functions in this pallet can emit.
	///
	/// Events are a simple means of indicating to the outside world (such as dApps, chain explorers
	/// or other users) that some notable update in the runtime has occurred. In a FRAME pallet, the
	/// documentation for each event field and its parameters is added to a node's metadata so it
	/// can be used by external interfaces or tools.
	///
	///	The `generate_deposit` macro generates a function on `Pallet` called `deposit_event` which
	/// will convert the event type of your pallet into `RuntimeEvent` (declared in the pallet's
	/// [`Config`] trait) and deposit it using [`frame_system::Pallet::deposit_event`].
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
	///
	/// Errors tell users that something went wrong so it's important that their naming is
	/// informative. Similar to events, error documentation is added to a node's metadata so it's
	/// equally important that they have helpful documentation associated with them.
	///
	/// This type of runtime error can be up to 4 bytes in size should you want to return additional
	/// information.
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

	/// The pallet's dispatchable functions ([`Call`]s).
	///
	/// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	/// These functions materialize as "extrinsics", which are often compared to transactions.
	/// They must always return a `DispatchResult` and be annotated with a weight and call index.
	///
	/// The [`call_index`] macro is used to explicitly
	/// define an index for calls in the [`Call`] enum. This is useful for pallets that may
	/// introduce new dispatchables over time. If the order of a dispatchable changes, its index
	/// will also change which will break backwards compatibility.
	///
	/// The [`weight`] macro is used to assign a weight to each call.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a single u32 value as a parameter, writes the value
		/// to storage and emits an event.
		///
		/// It checks that the _origin_ for this call is _Signed_ and returns a dispatch
		/// error if it isn't. Learn more about origins here: <https://docs.substrate.io/build/origins/>
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::do_something())]
		pub fn create_tag(origin: OriginFor<T>, name: BoundedVec<u8, T::TagNameLimit>) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			// Amount to deposit. Comes from configuration but it's good practice to store any amount
			// reserved at any point.
			let deposit = T::TagDepositAmount::get();

			// Try reserving the amount. This function naturally fails if the account lacks funds.
			T::Currency::reserve(&who, deposit)?;

			// By this point all checks should have been done (enough balance, no duplication, etc)
			// TODO: There's a check still missing atp: whether there's space for more tags.

			// Get the next available index and update the counter
			let index = match TagIndex::<T>::get() {
				// Return an error if the value has not been set.
				None => {
					let new_index = 1;
					TagIndex::<T>::put(new_index + 1);
					new_index
				},
				Some(last_index) => {
					// Increment the value read from storage. This will cause an error in the event
					// of overflow.
					let new_index = last_index.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					TagIndex::<T>::put(new_index);
					last_index
				},
			};

			TagMap::<T>::insert(index, (name, Some(who.clone()), deposit));

			// Emit the corresponding event.
			Self::deposit_event(Event::TagCreated {
				index, who, deposit
			});

			// Return a successful `DispatchResult`
			Ok(())
		}

		/// Destroys a tag from the chain
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::cause_error())]
		pub fn destroy_tag(origin: OriginFor<T>, tag_index: u64) -> DispatchResult {
			// TODO: I'm not familiar with the transactionality guarantees that apply here.
			// If this were multithreaded, and multiple calls were executed at the same time,
			// an account could remove from its reserve multiple times for destroying a single tag.

			// Check that the extrinsic was signed and get the signer.
			let who = ensure_signed(origin)?;

			let (_name, creator_opt, deposit) = TagMap::<T>::try_get(tag_index).map_err(|()|Error::<T>::InvalidTag)?;

			if creator_opt.as_ref() != Some(&who) {
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
