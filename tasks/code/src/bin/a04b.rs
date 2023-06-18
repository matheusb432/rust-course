// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * 1. Use a variable set to any integer
// * 2. Use a match expression to determine which message to display
// * 3. Use an underscore (_) to match on any value

fn main() {
    let inty = 42;

    match inty {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other"),
    }
}
