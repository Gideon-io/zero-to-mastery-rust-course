// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;
fn main() {
    let mut list = HashMap::new();

    list.insert("chairs", 5);
    list.insert("beds", 3);
    list.insert("tables", 2);
    list.insert("couches", 0);

    let mut total_quantity = 0;

    for (key,value) in list.iter() {
        total_quantity = total_quantity + value;

        let stock_count = if value == &0 {
            "zero stock".to_owned()
        }
        else {
            format!("{:?}", value)
        };
        println!("Item: {} || Stock: {:?}", key, stock_count);
    }

    println!("Total stock using the sum() method: {:?} || Total using the alt methid: {:?}", &list.values().sum::<i32>(), total_quantity);

}
