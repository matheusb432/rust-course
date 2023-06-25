// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io::Error;

use billing_app::bill::{Bill, BillAction, BillCli};
use billing_app::input::{get_input, input_loop, input_mut_loop, print_help};

fn handle_command(input: &str, mut store: &mut Vec<Bill>) -> Result<(), String> {
    let cli_command = BillCli::new_result(input)?;
    // ? With PowerState as Option
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
