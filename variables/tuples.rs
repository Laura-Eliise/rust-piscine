#[derive(Debug)]
pub struct Student (i32, String, String);

fn main() {
	let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
	println!("Student: {:?}", student);
	println!("Student first name: {}", first_name(&student));
	println!("Student last name: {}", last_name(&student));
	println!("Student Id: {}", id(&student));
}

pub fn id(student: &Student) -> i32 {student.0}

pub fn first_name(student: &Student) -> String {
    let Student(_, name, _) = student;
    return name.to_string()
}

pub fn last_name(student: &Student) -> String {
    let Student(_, _, surname) = student;
    return surname.to_string()
}