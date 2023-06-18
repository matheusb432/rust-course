// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * 1. Use a mutable integer variable
// * 2. Use a while statement
// * 3. Print the variable within the while loop
// * 4. Do not use break to exit the loop

fn main() {
    // * 1.
    let mut i = 5;

    // * 2. 4.
    while i >= 1 {
        // * 3.
        println!("i: {i:?}");

        i -= 1;
    }
    println!("done!")
}
