use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch add new member to default club.
		assert_ok!(TemplateModule::add_member(Origin::signed(1), 1));
		// Remove mumber from club given for a given club id.
		assert_ok!(TemplateModule::remove_member(Origin::signed(1), 1));
	});
}
