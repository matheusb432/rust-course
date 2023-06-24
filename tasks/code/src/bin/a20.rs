// Topic: User input
//
// Requirements:
// * 1. Verify user input against pre-defined keywords
// * 2. The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * 3. If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * 4. If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use code::input::{input_loop, print_help};
// NOTE Importing the PowerState enum and constants from the power_state module
use code::power_state::{PowerState, HIBERNATE, OFF, REBOOT, SHUTDOWN, SLEEP};

fn handle_power_state(input: &str) -> Result<(), String> {
    // NOTE The `?` operator on a Result essentially does optional chaining
    PowerState::from_str(input)?.print();

    Ok(())
}

fn main() {
    let commands = vec![OFF, SLEEP, REBOOT, SHUTDOWN, HIBERNATE];
    print_help(&commands);

    match input_loop(handle_power_state, &commands) {
        Ok(_) => {
            println!("Exiting program...");
        }
        Err(err) => println!("Error: {err:?}"),
    }
}
