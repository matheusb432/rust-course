// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * 1. Use a mutable integer variable
// * 2. Use a loop statement
// * 3. Print the variable within the loop statement
// * 4. Use break to exit the loop

fn main() {
    // * 1.
    let mut i = 1;
    // * 2.
    loop {
        // * 3.
        println!("{i:?}");
        i += 1;

        // * 4.
        if i > 4 {
            break;
        }
    }
}
