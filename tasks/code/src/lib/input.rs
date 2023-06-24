// * a20.rs

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
    let cmds = vec![
        "off".to_owned(),
        "sleep".to_owned(),
        "reboot".to_owned(),
        "shutdown".to_owned(),
        "hibernate".to_owned(),
    ];
    loop {
        println!("Enter a command:");
        let input = get_input()?;

        match input.as_str() {
            EXIT => break,
            HELP => print_help(cmds),
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

fn print_help(commands: Vec<String>) {
    // println!(
    //     "\nCommands:\n\
    //     - off\n\
    //     - sleep\n\
    //     - reboot\n\
    //     - shutdown\n\
    //     - hibernate\n\
    //     To exit the program:\n\
    //     - exit\n\
    //     To see the commands again:\n\
    //     - help\n"
    // );
    // NOTE Shadowing `commands`
    let commands = commands
        .iter()
        .fold("".to_owned(), |acc, curr| format!("\n{curr}- {acc}\n"));

    println!(
        "\nCommands:\n\
        {commands}
        To exit the program:\n\
        - exit\n\
        To see the commands again:\n\
        - help\n"
    );
}

fn main() {
    let cmds = vec![
        "off".to_owned(),
        "sleep".to_owned(),
        "reboot".to_owned(),
        "shutdown".to_owned(),
        "hibernate".to_owned(),
    ];
    print_help(cmds);

    match input_loop() {
        Ok(_) => {
            println!("Exiting program...");
        }
        Err(err) => println!("Error: {err:?}"),
    }
}
