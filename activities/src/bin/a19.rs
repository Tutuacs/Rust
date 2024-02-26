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

    let mut stock_items: HashMap<&str, i32> = HashMap::new();
    stock_items.insert("Chair", 5);
    stock_items.insert("Beds", 3);
    stock_items.insert("Tables", 2);
    stock_items.insert("Couches", 0);

    let mut total: i32 = 0;

    for (item, quantity) in stock_items.iter() {
        total += quantity;
        match (item, quantity) {
            (no_item, 0) => println!("{} out of stock",no_item),
            (any_item, any_quantity) => println!("{} units of {}", any_quantity, any_item),
        }
    }

    println!("\ntotal stock: {}", total);
    // format! -> function to concat strings/variables
}
