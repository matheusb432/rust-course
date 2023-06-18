// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * 1. Use a function to add two numbers together
// * 2. Use a function to display the result
// * 3. Use the "{:?}" token in the println macro to display the result

// * 1.
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

// * 2.
fn display_res(res: i32) {
    // * 3.
    println!("Result: {res:?}")
}

fn main() {
    let res = sum(1, 2);
    display_res(res);
}
