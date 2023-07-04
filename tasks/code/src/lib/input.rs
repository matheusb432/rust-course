// * a20.rs

const EXIT: &str = "exit";
const HELP: &str = "help";

use std::io;

pub fn get_input() -> io::Result<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    Ok(buffer.trim().to_owned())
}

// * 1.
// NOTE Reusable input loop function that accepts a function and commands to handle it in a generic way
pub fn input_loop(
    handle_input: impl Fn(&str) -> Result<(), String>,
    commands: &Vec<&str>,
) -> Result<(), io::Error> {
    loop {
        println!("Enter a command:");
        let input = get_input()?;

        match input.as_str() {
            EXIT => break,
            HELP => print_help(commands),
            input => handle_input(input).unwrap_or_else(|err| println!("\nERROR: {err}\n")),
        }
    }
    Ok(())
}

pub fn print_help(commands: &Vec<&str>) {
    // NOTE Shadowing `commands` so it's more memory efficient and there's not need to create a `commands_str` variable
    // * Creating via join(), is it more memory efficient this way?
    // let commands: String = commands
    //     .iter()
    //     .map(|command| format!("- {}\n", command))
    //     .collect::<Vec<String>>()
    //     .join("");
    // ? Creating via fold()
    let commands: String = commands
        .iter()
        .fold("".to_owned(), |acc, curr| format!("- {curr}\n{acc}"));

    println!(
        "\nCommands:\n\
        {commands}\
        To exit the program:\n\
        - exit\n\
        To see the commands again:\n\
        - help\n"
    );
}
