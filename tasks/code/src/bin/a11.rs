// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * 1. Use a struct for the grocery item
// * 2. Use two i32 fields for the quantity and id number
// * 3. Create a function to display the quantity, with the struct as a parameter
// * 4. Create a function to display the id number, with the struct as a parameter

// * 1. 2.
struct GroceryItem {
    qty: i32,
    id: i32,
}

fn display_item(item: &GroceryItem) {
    // NOTE since `item` is already a reference to a GroceryItem, `&item` is not necessary
    display_qty(item);
    display_id(item);
}

// * 3.
fn display_qty(item: &GroceryItem) {
    println!("Quantity is {:?}", item.qty)
}

// * 4.
fn display_id(item: &GroceryItem) {
    println!("Id is {:?}", item.id)
}

fn main() {
    let googly_eyes = GroceryItem { id: 1, qty: 50 };
    let glue = GroceryItem { id: 2, qty: 1 };

    // display_id(&googly_eyes);
    // display_qty(&googly_eyes);

    // display_id(&glue);
    // display_qty(&glue);
    display_item(&googly_eyes);
    display_item(&glue);
}
