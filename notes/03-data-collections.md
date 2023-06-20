# Rust Data Collections

## Impl

Implementations allows you to define methods that operate on instances of a struct or an enum, as well as associated functions that do not operate on an instance but are associated with the type. Additionally, it's used for implementing traits for specific types.

Implementations enable the usage of OOP patterns.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

```

## Vector

Vectors allow you to store lists of data. Vectors can only store values of the same type.

Can add, remove, update elements.

```rust
// The vec! macro expands to something similar to creating the vector and pushing the values into it
let my_numbers = vec![1, 2, 3];

let mut my_numbers = Vec::new();
my_numbers.push(1);
my_numbers.push(2);
my_numbers.push(3);
my_numbers.pop();
my_numbers.len(); // 2

// Acessing a vector value using the slice notation, by it's index
let two = my_numbers[1];

// Iterating in vector values
for num in my_numbers {
    println!("{:?}", num);
}

```

## String and &str

String is the owned data type, and &str is the borrowed data type.

Must use an owned String to store it in a struct, and it's more efficient to pass &str as a function parameter.

```rust
fn print_it(input: &str) {
    println!("{}", input);
}

fn main() {
    // The string in quotes is automatically borrowed as a &str
    print_it("a string slice");

    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another owned string");

    // Passing String types as their references
    print_it(&owned_string);
    print_it(&another_owned);
}
```

## Enums

Enums are not limited to just plain variants, each variant can optionally contain additional data

The additional data is required to create variants on enums that specify a type.

```rust
enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    // Scroll can represent the mouse scroll amount
    Scroll(i32),
    // Move can represent the mouse movement in x and y
    Move(i32, i32),
}


enum PromoDiscount {
    NewUser,
    Holiday(String),
}

enum Discount {
    Percent(i32),
    Flat(i32),
    // It's possible to have enums within enums
    Promo(PromoDiscount),
    Custom(String),
}
```