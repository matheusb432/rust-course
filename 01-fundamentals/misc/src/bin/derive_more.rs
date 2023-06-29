#![allow(dead_code, unused_imports, unused_variables)]

use derive_more::{Display, From, Into, IntoIterator};

fn main() {
    #[derive(Display)]
    #[display(fmt = "Item: {}, Quantity: {}", item, qty)]
    struct Order {
        item: String,
        qty: usize,
    }
    let order = Order {
        item: "chocolate".to_string(),
        qty: 10,
    };
    println!("{}", order);

    #[derive(Display)]
    enum GroceryItem {
        Apple,
        #[display(fmt = "Chocolate")]
        Chocolate,
        #[display(fmt = "Ice cream: {} scoops", _0)]
        IceCream(usize),
    }
    let item = GroceryItem::IceCream(3);
    println!("{}", item);

    // NOTE With From derive from derive_mode, it's possible to automatically implement the From trait for a type.
    #[derive(From)]
    struct UserId(i32);

    let user_id: UserId = 15.into();
    let user_id = UserId::from(15);

    println!("User id: {}", user_id.0);

    #[derive(From)]
    struct Coordinate {
        x: i32,
        y: i32,
    }

    // NOTE Will receive fields as a tuple in the same order as they are declared in the struct
    let coordinate = Coordinate::from((10, 20));

    println!("x: {}, y: {}", coordinate.x, coordinate.y);

    #[derive(From, Debug)]
    enum Material {
        Flooring(usize, usize),
        Wood(usize),
    }

    let floor = Material::from((5, 6));
    let wood = Material::from(7);

    println!("Flooring: {:?}", floor);
    println!("Wood: {:?}", wood);

    #[derive(IntoIterator)]
    struct Passengers {
        // NOTE Adding move, borrow and mutable borrow iterators for this struct
        #[into_iterator(owned, ref, ref_mut)]
        names: Vec<String>,
    }
    let passengers = Passengers {
        names: vec!["Alice".to_string(), "Bob".to_string()],
    };
    for name in &passengers {
        println!("Passenger: {}", name);
    }
}
