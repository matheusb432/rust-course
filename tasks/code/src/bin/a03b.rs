// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable is > 5, < 5, or == 5, respectively
//
// Notes:
// * 1. Use a variable set to any integer value
// * 2. Use an if..else if..else block to determine which message to display
// * 3. Use the println macro to display messages to the terminal

fn display_msg(val: i32) {
    // * 2.
    if val == 5 {
        // * 3.
        println!("{val} == 5")
    } else if val > 5 {
        println!("{val} > 5")
    } else {
        println!("{val} < 5")
    }
}

fn main() {
    // * 1.
    let val = 5;

    display_msg(val);
    display_msg(10);
    display_msg(4);
}
