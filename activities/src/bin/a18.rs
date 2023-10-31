// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message
#[derive(Debug)]
struct Adult {
    name: String,
    age: i32,
}

/*
Returning a Result type is always good for a function as it ensures we're returning an Ok variant as it performs all the check
inside the function. All the checks are done inside the function so you dont need to check it again. This is good for optimization
and makes the code easier to work with
*/

impl Adult {
    fn new(name: String, age: i32) -> Result<Self, String> {
        match age >= 21 {
            true => Ok( Self {name, age} ),
            false => Err("Under the age of 21".to_string())
        }
    }
}
fn main() {
    let albie = Adult::new("albie".to_owned(),18);

    println!("{:?}", albie);

    match albie {
        Ok(result) => println!("User created: {:?}", result),
        Err(msg) => println!("{:?}", msg)
    }

    let taras = Adult::new("taras".to_owned(), 21);

    println!("{:?}", taras);

    match taras {
        Ok(result) => println!("Name: {} || Age: {}", result.age, result.name), //just used a different method here compared to albie
        Err(msg) => println!("{:?}", msg)
    }
}
