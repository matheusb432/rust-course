use billing_app::bill::{Bill, BillAction, BillCli};
use billing_app::input::{get_input, input_mut_loop, print_help};

fn handle_command(input: &str, mut store: &mut Vec<Bill>) -> Result<(), String> {
    let cli_command = BillCli::new_result(input)?;
    match cli_command {
        BillCli::Add => {
            let new_bill = loop {
                println!("Add a bill:");

                match handle_add() {
                    Ok(b) => break b,
                    Err(err) => {
                        println!("ERROR: {err:?}");
                        println!("Trying again..");
                        continue;
                    }
                };
            };

            Bill::dispatch_vec(&mut store, BillAction::Add(new_bill));
        }
        BillCli::List => {
            println!("List of bills:");
            dbg!(store);
        }
        BillCli::Back => {
            println!("Going back...");
        }
        _ => {
            println!("Not implemented!")
        }
    }
    Ok(())
}

fn handle_add() -> Result<Bill, String> {
    println!("1. Enter the bill name:");

    let name = match get_input() {
        Ok(input) => {
            Bill::validate_name(input.as_str())?;
            input
        }
        Err(err) => return Err(err.to_string()),
    };

    println!("2. Enter bill price:");

    let price = match get_input() {
        Ok(input) => {
            let value = match input.parse::<f64>() {
                Err(err) => return Err(err.to_string()),
                Ok(value) => value,
            };
            Bill::validate_price(&value)?;
            value
        }
        Err(err) => return Err(err.to_string()),
    };

    Bill::new(name, price)
}

fn main() {
    let mut vec_store = Bill::create_vec_store();
    let commands = BillCli::list_commands();
    let handle_input = |input: &str| handle_command(input, &mut vec_store);

    print_help(&commands);
    match input_mut_loop(handle_input, &commands) {
        Ok(_) => {
            println!("Exiting program...");
        }
        Err(err) => {
            println!("ERROR: {err:?}");
        }
    }
}
