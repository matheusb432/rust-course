use std::collections::HashMap;

// NOTE Defining a module
mod greet {
    // NOTE a module is isolated from the rest of the program, so importing the module here is necessary
    use std::collections::HashMap;

    // NOTE the `pub` keyword makes the module public
    pub fn hello() {
        println!("Hello, world!");
    }

    pub fn goodbye() {
        println!("Goodbye, world!");
    }

    // NOTE by default all items in a module are private, so the `pub` keyword is necessary to make them public
    fn private_fn() {
        println!("This is a private function");
    }
}

mod math {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    // NOTE `use` keyword to bring a module into scope, it's possible to set them inline
    use greet::*;
    hello();
    greet::goodbye();
}
