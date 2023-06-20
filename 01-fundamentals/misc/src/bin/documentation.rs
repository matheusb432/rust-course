/// A favorite color.
enum Color {
    Red,
    Blue,
}

/// A piece of mail
struct Mail {
    /// The address
    address: String,
}

// NOTE adding documentation block with `///`
/// Adds two numbers together.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let values = vec![1, 2, 3, 4];

    match values.is_empty() {
        true => println!("No values"),
        false => println!("Has values"),
    }
}

// NOTE To generate documentation as an HTML file, run `cargo doc --open`
