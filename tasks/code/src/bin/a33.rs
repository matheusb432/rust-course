// Topic: Lifetimes & Functions
//
// Summary:
// Create a program that compares which string is longer (highest length).
//
// Requirements:
// * The comparison must be done using a function named `longest`
// * No data may be copied (cannot use .to_owned() or .to_string())
// * If both strings are the same length, the first one should be returned

// NOTE Returning a &str with a function by leveraging lifetimes
// ? It's necessary to specify it's lifetime so that
// ? the compiler can know that the return value is one of the parameters
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if y.len() > x.len() {
        y
    } else {
        x
    }
}

fn main() {
    let short = "this is message";
    let long = "this is a long message";
    println!("{}", longest(short, long))
}
