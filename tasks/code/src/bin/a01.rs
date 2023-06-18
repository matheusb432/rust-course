// Topic: Functions
//
// Program requirements:
// * Display first and last name
//
// Notes:
// * 1. Use a function to display your first name
// * 2. Use a function to display your last name
// * 3. Use the println macro to display messages

// * 1.
fn display_first_name(first: &str) {
    println!("First name: {first}");
}

// * 2.
fn display_last_name(last: &str) {
    println!("Last name: {last}");
}

// NOTE accepting parameters in Rust
fn display_name(first: &str, last: &str) {
    display_first_name(first);
    display_last_name(last);
    // * 3.
    println!("My name is {first} {last}")
}

fn main() {
    display_name("Jake", "Weary")
}
