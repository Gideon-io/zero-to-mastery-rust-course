// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavour {
    Sweet,
    Sour,
}

struct Drink {
    flavour: Flavour,
    oz: i32,
}

fn print_colour(value: Drink) {
    match value.flavour {
        Flavour::Sweet => println!("Sweet"),
        Flavour::Sour => println!("Sour")
    }
    println!("{}", value.oz);
}
fn main() {
    let sweet_drink = Drink {
        flavour: Flavour::Sweet,
        oz: 3
    };
    print_colour(sweet_drink);
}
