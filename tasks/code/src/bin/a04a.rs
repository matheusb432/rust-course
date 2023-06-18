// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * 1. Use a variable set to either true or false
// * 2. Use a match expression to determine which message to display

fn main() {
    // * 1.
    let booly = true;

    // * 2.
    match booly {
        true => println!("it's true"),
        false => println!("it's false"),
    }
}
