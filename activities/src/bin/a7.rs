// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Color {
    Blue,
    Red,
}

fn print_colour(result: Color) {
    match result {
        Color::Blue => println!("blue"),
        Color::Red => println!("Red"),
    }

}
fn main() {
    let value = Color::Blue;

    print_colour(value);
}
