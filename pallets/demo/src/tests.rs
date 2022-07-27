use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]

fn test_create_student() {
	new_test_ext().execute_with(|| {
		assert_ok!(Demo::create_student(Origin::signed(1), b"Huy Duc".to_vec(), 24));
		assert_eq!(Demo::student_id(), 1);
	});
}
#[test]
fn correct_error_for_young_student() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		assert_noop!(Demo::create_student(Origin::signed(1), b"Huy Duc".to_vec(), 18), Error::<Test>::TooYoung);
	});
}
