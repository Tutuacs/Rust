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
    locker: Option<i32>
}

impl Student {
    fn new(name: String, locker: Option<i32>) -> Self {
        Self {
            name,
            locker,
        }
    }

    fn print(&self) {
        println!("\nStudents name: {}", self.name);
        match &self.locker {
            Some(number) => println!("the student have locker: {:?}", number), 
            None => println!("the student dont have"), 
        }
    }
}


fn main() {

    let name: String = "Arthur".into();
    let locker = Some(3);
    let student: Student = Student::new(name, locker);

    student.print();
}
