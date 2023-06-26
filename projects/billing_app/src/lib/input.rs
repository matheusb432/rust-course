const EXIT: &'static str = "exit";
const HELP: &'static str = "help";

use std::io;

pub fn get_input() -> io::Result<String> {
    let mut buffer = String::new();

    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please enter your data again");
    }

    Ok(buffer.trim().to_owned())
}
// ? Alternative with Option
// pub fn get_input() -> Option<String> {
//     let mut buffer = String::new();

//     while io::stdin().read_line(&mut buffer).is_err() {
//         println!("Please enter your data again");
//     }

//     let input = buffer.trim().to_owned();
//     if &input == "" {
//         None
//     } else {
//         Some(input)
//     }
// }

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

pub fn input_mut_loop(
    mut handle_input: impl FnMut(&str) -> Result<(), String>,
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
