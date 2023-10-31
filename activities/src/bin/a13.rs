// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector
struct Number {
    number: i32
}
fn main() {
    let list = vec![Number{number:10}, Number{number:20}, Number{number:30}, Number{number:40}];

    for value in &list { 

        match value.number {
            30 => println!("thirty"),
            _ => println!("{}", value.number)
        }
        
        
    }

    println!("length of the list: {}", &list.len());
}
