use crate::{mock::*, Error, Person, Name, Job, Age};
use frame_support::{assert_noop, assert_ok};

#[test]
fn add_person_to_registry() {
	new_test_ext().execute_with(|| {
		let name: Name<Test> = <Name<Test>>::default();
		let age: Age = 34u32.into();
		let job: Job = Job::Programmer;
		assert_ok!(Registry::add_registry(RuntimeOrigin::signed(1), name.clone(), age, job.clone()));
		let expected_person: Person<Test> = Registry::people_registry(1).expect("Was added to Registry");
		assert_eq!(
			expected_person,
			Person::<Test> { name, age, job }
		);
	});
}

#[test]
fn add_person_to_old_to_registry() {
	new_test_ext().execute_with(|| {
		let name: Name<Test> = "Andrew"
			.as_bytes()
			.to_vec()
			.try_into()
			.expect("Can Convert this Name to Bounded Vec");
		assert_noop!(
			Registry::add_registry(RuntimeOrigin::signed(1), name, 35u32, Job::Programmer),
			Error::<Test>::MaxAgeExceeded,
		);
	});
}
