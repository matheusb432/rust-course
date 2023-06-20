fn main() {
    let mut x = 0;

    println!("loop:");
    // NOTE loop is an infinite loop
    loop {
        if x == 5 {
            break;
        }
        // NOTE Ternary expression in rust:
        let spacing: &str = if x == 4 { "\n" } else { ", " };

        print!("{x:?}{spacing}");
        x += 1;
    }
    println!("while:");

    // NOTE while runs while the condition is true
    while x > 0 {
        let spacing: &str = if x == 1 { "\n" } else { ", " };

        print!("{x:?}{spacing}");
        x -= 1;
    }

    let mut i: i32 = 3;
    loop {
        println!("i: {i:?}");
        i -= 1;
        if i == 0 {
            break;
        }
    }

    let mut j: i32 = 1;

    while j <= 3 {
        println!("j: {j:?}");
        j += 1;
    }
}
