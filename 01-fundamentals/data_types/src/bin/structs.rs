struct GroceryItem {
    // NOTE u32 is an unsigned 32-bit integer, it can only be positive
    stock: u32,
    // NOTE f64 is the float type
    price: f64,
}

fn main() {
    let cereal = GroceryItem {
        stock: 10,
        price: 3.99,
    };

    println!("stock -> {:?} & price -> {:?}", cereal.stock, cereal.price);
}
