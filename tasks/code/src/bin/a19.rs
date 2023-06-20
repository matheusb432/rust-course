// Topic: HashMap
//
// Requirements:
// * 1. Print the name and number of items in stock for a furniture store
// * 2. If the number of items is 0, print "out of stock" instead of 0
// * 3. The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * 4. Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
enum Furniture {
    Chair,
    Bed,
    Table,
    Couch,
}

fn main() {
    let mut store: HashMap<Furniture, i32> = HashMap::new();
    let mut sum = 0;

    // * 3.
    // NOTE using an enum as the HashMap key, needs to derive from the `Hash, Eq, PartialEq` macros
    store.insert(Furniture::Chair, 5);
    store.insert(Furniture::Bed, 3);
    store.insert(Furniture::Table, 2);
    store.insert(Furniture::Couch, 0);

    for (furniture, items_count) in store.iter() {
        sum += items_count;
        // * 1. 2.
        match items_count {
            &0 => println!("{:?}: out of stock", furniture),
            count => println!("{:?}: {:?} items in stock", furniture, count),
        }
    }

    // * 4.
    println!("Total items: {:?}", sum);
    println!(
        "w/ .values().fold(): {:?}",
        // NOTE values() returns an iterator over the references of the HashMap values
        // NOTE fold() reduces an iterable to a single value
        store.values().fold(0, |acc, curr| acc + curr)
    );
}
