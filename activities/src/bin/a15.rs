// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
enum Tier {
    Backstage(String, f64),
    Standard(f64),
    Vip(String, f64)
}
fn main() {
    let ticket_list = vec![
        Tier::Backstage("Gideon".to_owned(), 15.0),
        Tier::Standard(10.0),
        Tier::Vip("Lee".to_owned(), 23.0)
    ];

    for person in ticket_list {
        match person {
            Tier::Backstage(name, price) => println!("BACKSTAGE || Name: {} || Price: {}", name, price),
            Tier::Standard(price) => println!("STANDARD || Price: {}", price),
            Tier::Vip(name, price) => println!("vip || Name: {} || Price: {}", name, price)
        }
    }
}
