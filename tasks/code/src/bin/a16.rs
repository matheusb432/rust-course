// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * 1. Use a struct containing the student's name and locker assignment
// * 2. The locker assignment should use an Option<i32>

#![allow(dead_code)]

// ? 1.
struct Student {
    name: String,
    // ? 2.
    locker: Option<i32>,
}

fn main() {
    let first = Student {
        name: "student 1".to_owned(),
        locker: Some(10),
    };
    let second = Student {
        name: "student 2".to_owned(),
        locker: None,
    };
    let third = Student {
        name: "student 3".to_owned(),
        locker: Some(9),
    };
    let students = vec![first, second, third];

    for student in students {
        match student.locker {
            Some(val) => println!("locker number: {:?}", val),
            None => println!("no locker assigned"),
        }
    }
}
