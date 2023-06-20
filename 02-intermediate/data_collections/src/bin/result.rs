#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

// NOTE This type means that the success data is MenuChoice and the error data is String
fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_string()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

// NOTE the unit type () is used to indicate that the function does not return a value on Ok()
fn pick_choice(input: &str) -> Result<(), String> {
    // NOTE the `?` will match the result to place the inner data on `choice`, or return the error
    let choice = get_choice(input)?;

    print_choice(&choice);
    Ok(())
}

fn main() {
    for item in ["mainmenu", "quit", "invalid"].iter() {
        match pick_choice(*item) {
            Err(e) => println!("Error! {:?}", e),
            _ => (),
        }
    }
}
