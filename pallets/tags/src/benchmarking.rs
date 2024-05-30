//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Tags;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use frame_support::traits::{Get, Currency};

macro_rules! bvec {
	($( $x:tt )*) => {
		TryInto::<BoundedVec<_, _>>::try_into(vec![$( $x )*]).unwrap()
	}
}

#[benchmarks]
mod benchmarks {
	use super::*;

	use sp_core::bounded::BoundedVec;
	use sp_std::vec;

	#[benchmark]
	fn create_tag() {
		let caller: T::AccountId = whitelisted_caller();
		let tag_index = TagIndex::<T>::get();
		let deposit = T::TagDepositAmount::get();
		let name = bvec![];

		T::Currency::make_free_balance_be(&caller, deposit + deposit);

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), name.clone());

		assert_eq!(TagMap::<T>::try_get(tag_index), Ok(
			(name.clone(), caller, deposit)
		));
	}

	#[benchmark]
	fn destroy_tag() {
		let caller: T::AccountId = whitelisted_caller();
		let tag_index = TagIndex::<T>::get();
		let deposit = T::TagDepositAmount::get();
		let name = bvec![];

		T::Currency::make_free_balance_be(&caller, deposit + deposit);

		Tags::<T>::create_tag(RawOrigin::Signed(caller.clone()).into(), name.clone()).unwrap();
		assert_eq!(TagMap::<T>::try_get(tag_index), Ok(
			(name.clone(), caller.clone(), deposit)
		));

		#[extrinsic_call]
		_(RawOrigin::Signed(caller), tag_index);

		assert_eq!(TagMap::<T>::get(tag_index), None);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
