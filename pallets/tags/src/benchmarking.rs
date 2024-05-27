//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Template;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::traits::Get;
use sp_core::bounded::BoundedVec;
use sp_std::vec::vec;

macro_rules! bvec {
	($( $x:tt )*) => {
		TryInto::<BoundedVec<_, _>>::try_into(vec![$( $x )*]).unwrap()
	}
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_tag() {
		let caller: T::AccountId = whitelisted_caller();
		let tag_index = TagIndex::<T>::get();
		let deposit = T::TagDepositAmount::get();
		let name = bvec![];

		#[extrinsic_call]
		create_tag(RawOrigin::Signed(caller), name.clone());

		assert_eq!(TagMap::<T>::try_get(tag_index), Ok(
			(name.clone(), 1, deposit)
		));
	}

	#[benchmark]
	fn destroy_tag() {
		let caller: T::AccountId = whitelisted_caller();
		let tag_index = TagIndex::<T>::get();
		let deposit = T::TagDepositAmount::get();
		let name = bvec![];

		create_tag(RawOrigin::Signed(caller), name.clone());
		assert_eq!(TagMap::<T>::get(tag_index), Some(
			(name.clone(), 1, deposit)
		));

		#[extrinsic_call]
		destroy_tag(RawOrigin::Signed(caller), tag_index);

		assert_eq!(TagMap::<T>::get(tag_index), None);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
