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
struct Locker {
    name: String,
    locker: Option<i32>
}
fn main() {
    let alan = Locker {name: "alan".to_owned(), locker: Some(5)};

    println!("Name: {}", alan.name);
    match alan.locker {
        Some(num) => println!("{}", num),
        None => println!("No locker assigned")
    }
}
