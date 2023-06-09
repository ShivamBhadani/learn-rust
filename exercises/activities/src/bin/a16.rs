// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

impl Student {
    fn new(name: &str, locker: Option<i32>) -> Self {
        Self {
            name: name.to_string(),
            locker,
        }
    }
}

fn main() {
    let students = vec![
        Student::new("Alice", Some(1)),
        Student::new("Bob", None),
        Student::new("Carol", Some(3)),
    ];

    for student in students {
        match student.locker {
            Some(locker) => println!("{} is assigned locker {}", student.name, locker),
            None => println!("{} is not assigned a locker", student.name),
        }
    }

}

