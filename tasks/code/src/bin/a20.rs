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

const EXIT: &'static str = "exit";
const HELP: &'static str = "help";

// NOTE Importing the PowerState enum from the power_state module
use code::power_state::PowerState;
use std::io;

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

// * 1.
fn input_loop() -> Result<(), io::Error> {
    loop {
        println!("Enter a command:");
        let input = get_input()?;

        match input.as_str() {
            EXIT => break,
            HELP => print_help(),
            input => handle_power_state(input).unwrap_or_else(|err| println!("\nERROR: {err}\n")),
        }
    }
    Ok(())
}

fn handle_power_state(input: &str) -> Result<(), String> {
    // NOTE The `?` operator on a Result essentially does optional chaining
    PowerState::from_str(input)?.print();

    Ok(())
}

fn print_help() {
    println!(
        "\nCommands:\n\
        - off\n\
        - sleep\n\
        - reboot\n\
        - shutdown\n\
        - hibernate\n\
        To exit the program:\n\
        - exit\n\
        To see the commands again:\n\
        - help\n"
    );
}

fn main() {
    print_help();

    match input_loop() {
        Ok(_) => {
            println!("Exiting program...");
        }
        Err(err) => println!("Error: {err:?}"),
    }
}
