// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * 1. Use an enum with color names as variants
// * 2. Use a function to print the color name
// * 3. The function must use the enum as a parameter
// * 4. Use a match expression to determine which color name to print

// * 1.
enum Color {
    Orange,
    Purple,
    Pink,
}

// * 2. 3.
fn print_color(color: Color) {
    // * 4.
    match color {
        Color::Orange => println!("Orange!"),
        Color::Purple => println!("Purple!"),
        Color::Pink => println!("Pink!"),
    }
}

fn main() {
    let orange = Color::Orange;
    let purple = Color::Purple;

    print_color(orange);
    print_color(purple);
    print_color(Color::Pink);
}
