fn main() {
    // :)
    println!("Hello World!");

    let mut x = 0;

    println!("loop:");
    loop {
        if x == 5 {
            break;
        }
        // NOTE Ternary expression in rust:
        let spacing: &str = if x == 4 { "" } else { ", " };

        print!("{x:?}{spacing}");
        x += 1;
    }
    println!("\nwhile:");

    while x > 0 {
        let spacing: &str = if x == 1 { "" } else { ", " };

        print!("{x:?}{spacing}");
        x -= 1;
    }

    let my_favorite_color = "orange";

    println!("\nMy favorite color: {my_favorite_color:?}");
}
