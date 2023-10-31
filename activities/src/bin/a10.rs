// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print
fn print(param: bool) {
    match param {
        true => println!("Its big"),
        false => println!("Its snmall")
    }
}
fn main() {
    let value = 55;

    let if_else_value = value > 100;

    print(if_else_value);
    
}
