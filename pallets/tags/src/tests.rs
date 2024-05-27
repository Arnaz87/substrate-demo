use crate::{mock::*, Error, Event, TagIndex, TagMap};
use frame_support::{assert_ok, assert_noop, traits::Currency};
use sp_core::bounded::BoundedVec;

macro_rules! bvec {
	($( $x:tt )*) => {
		TryInto::<BoundedVec<_, _>>::try_into(vec![$( $x )*]).unwrap()
	}
}

#[test]
fn create_and_destroy_tag() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Should be the first index
		let tag_index = TagIndex::<Test>::get();
		assert_eq!(tag_index, 0);

		// Give some amount to the account so the test can run
		Balances::make_free_balance_be(&1, 100);

		let name = bvec![65, 66, 67];
		let deposit = TagDepositAmount::get();

		// Dispatch a signed extrinsic.
		assert_ok!(TagModule::create_tag(RuntimeOrigin::signed(1), name.clone()));

		// Read pallet storage and assert an expected result.
		assert_eq!(TagMap::<Test>::try_get(tag_index), Ok(
			(name.clone(), 1, deposit)
		));

		// Ensure the generated event matches
		System::assert_last_event(Event::TagCreated { index: tag_index, who: 1, deposit }.into());

		// Assert the index advanced
		assert_eq!(TagIndex::<Test>::get(), tag_index + 1);

		// Invoke the destroy extrinsic
		assert_ok!(TagModule::destroy_tag(RuntimeOrigin::signed(1), tag_index));

		// Ensure the storage was modified and the event was emitted
		assert_eq!(TagMap::<Test>::try_get(tag_index), Err(()));
		System::assert_last_event(Event::TagDestroyed { index: tag_index, who: 1 }.into());
	});
}

#[test]
fn destroy_tag_by_owner() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Give some amount to the accounts so the test can run
		Balances::make_free_balance_be(&1, 100);
		Balances::make_free_balance_be(&2, 100);

		let name = bvec![65, 66, 67];
		let deposit = TagDepositAmount::get();
		let tag_index = 0;

		// Create tag
		assert_ok!(TagModule::create_tag(RuntimeOrigin::signed(1), name.clone()));
		System::assert_last_event(Event::TagCreated { index: tag_index, who: 1, deposit }.into());

		// Try to destroy with another user
		assert_noop!(TagModule::destroy_tag(RuntimeOrigin::signed(2), tag_index), Error::<Test>::NotAllowed);

		// Correctly destroy with the original creator of the tag
		assert_ok!(TagModule::destroy_tag(RuntimeOrigin::signed(1), tag_index));

		// Ensure the storage was modified and the event was emitted
		assert_eq!(TagMap::<Test>::try_get(tag_index), Err(()));
		System::assert_last_event(Event::TagDestroyed { index: tag_index, who: 1 }.into());
	});
}

#[test]
fn destroy_invalid_tag() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		// Give some amount to the accounts so the test can run
		Balances::make_free_balance_be(&1, 100);

		let tag_index = 1;

		// Try to destroy a tag that has never been created
		assert_noop!(TagModule::destroy_tag(RuntimeOrigin::signed(1), tag_index), Error::<Test>::InvalidTag);
	});
}