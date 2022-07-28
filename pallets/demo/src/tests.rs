use crate::{mock::*, Error, Gender};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_student_work_ok() {
	new_test_ext().execute_with(|| {
		let name = "Tien Trinh".as_bytes().to_vec();
		let age = 22;
		let student = TemplateModule::create_student(Origin::signed(1), name, age);
		assert_ok!(student);
	});
}

#[test]
fn create_student_correct_gender_female_value() {
	new_test_ext().execute_with(|| {
		let name = "Tien Trinh".as_bytes().to_vec();
		let student = TemplateModule::create_student(Origin::signed(1), name, 22);
		assert_ok!(student);

		let current_student = TemplateModule::student(0).unwrap();
		assert_eq!(current_student.gender, Gender::Female);
	});
}

#[test]
fn create_student_correct_gender_male_value() {
	new_test_ext().execute_with(|| {
		let name = "Tien Trinhh".as_bytes().to_vec();
		let age = 22;
		let student = TemplateModule::create_student(Origin::signed(1), name, age);
		assert_ok!(student);

		let current_student = TemplateModule::student(0).unwrap();
		assert_eq!(current_student.gender, Gender::Male);
	});
}

#[test]
fn correct_error_for_too_young() {
	new_test_ext().execute_with(|| {
		let name = "Tien Trinh".as_bytes().to_vec();
		let student = TemplateModule::create_student(Origin::signed(1), name, 18);
		assert_noop!(student, Error::<Test>::TooYoung);
	});
}

#[test]
fn create_student_work_with_correct_data() {
	new_test_ext().execute_with(|| {
		let name = "Tien Trinh".as_bytes().to_vec();
		let age = 22;
		let student = TemplateModule::create_student(Origin::signed(1), name.clone(), age);
		assert_ok!(student);

		let current_student = TemplateModule::student(0).unwrap();
		assert_eq!(current_student.name, name.clone());
		assert_eq!(current_student.age, age);
		assert_eq!(current_student.gender, TemplateModule::gen_gender(name).unwrap());
	});
}
