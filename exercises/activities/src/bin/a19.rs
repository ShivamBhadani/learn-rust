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
    let mut furniture = HashMap::new();
    furniture.insert(String::from("Chairs"), 5);
    furniture.insert(String::from("Beds"), 3);
    furniture.insert(String::from("Tables"), 2);
    furniture.insert(String::from("Couches"), 0);

    for (name, count) in &furniture {
        if *count == 0 {
            println!("{}: Out of stock", name);
        } else {
            println!("{}: {}", name, count);
        }
    }

    let mut total = 0;
    for (_, count) in &furniture {
        total += count;
    }
    println!("Total: {}", total);
}
