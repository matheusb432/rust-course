// Topic: New type pattern
//
// Requirements:
// * 1. Display the selected color of shoes, a shirt, and pants
// * 2. Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    Blue,
    Red,
}

struct Shoes(Color);
struct Shirt(Color);
struct Pants(Color);

impl Shoes {
    fn new(color: Color) -> Self {
        Shoes(color)
    }
}

impl Shirt {
    fn new(color: Color) -> Self {
        Shirt(color)
    }
}

impl Pants {
    fn new(color: Color) -> Self {
        Pants(color)
    }
}

// * 2.
fn print_shoes_color(shoes: Shoes) {
    println!("Shoes: {:?}", shoes.0);
}

fn print_shirt_color(shirt: Shirt) {
    println!("Shirt: {:?}", shirt.0);
}

fn print_pants_color(pants: Pants) {
    println!("Pants: {:?}", pants.0);
}

fn main() {
    // * 1.
    let shoes = Shoes::new(Color::Black);
    let shirt = Shirt::new(Color::Red);
    let pants = Pants::new(Color::Blue);

    print_shoes_color(shoes);
    print_shirt_color(shirt);
    print_pants_color(pants);
}
