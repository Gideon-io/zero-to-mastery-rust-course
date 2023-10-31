// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
struct Person {
    age: i32,
    name: String,
    color: String,
}
fn main() {

    let list = vec![
        Person{age: 4, name: "gideon".to_owned(), color: "blue".to_owned()}, 
        Person{age: 11, name: "Sarah".to_owned(), color: "yellow".to_owned()},
        Person{age: 15, name: "Ben".to_owned(), color: "green".to_owned()}
    ];

    for person in list {

        if person.age > 10 {
            println!("Name: {}", person.name);
            println!("Favourite Color: {}", person.color);
        }
    }
}
