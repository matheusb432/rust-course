// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * 1. Use a variable set to either true or false
// * 2. Use an if..else block to determine which message to display
// * 3. Use the println macro to display messages to the terminal

fn main() {
    // * 1.
    let can_display: bool = false;

    // * 2.
    // NOTE `== true` would be redundant
    if can_display {
        // * 3.
        println!("hello")
    } else {
        println!("goodbye")
    }
}
