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

use billing_app::bill::{Bill, BillAction, BillCli, BillStore};
use billing_app::input::{input_mut_loop, print_help};

fn handle_command(input: &str, store: &mut BillStore) -> Result<(), String> {
    let cli_command = BillCli::new_result(input)?;
    match cli_command {
        BillCli::Add => {
            let new_bill = loop {
                println!("Add a bill:");

                match create_bill() {
                    Ok(b) => break b,
                    Err(err) => {
                        if let Some(BillCli::Back) = BillCli::handle_err(&err) {
                            return Ok(());
                        }
                    }
                };
            };
            store.dispatch(BillAction::Add(new_bill));
        }
        BillCli::Remove => {
            if store.items().is_empty() {
                println!("No bills can be removed!");
                return Ok(());
            }

            store.print_list();
            let name_key = loop {
                match create_bill_key() {
                    Ok(bill_key) => break bill_key,
                    Err(err) => {
                        if let Some(BillCli::Back) = BillCli::handle_err(&err) {
                            return Ok(());
                        }
                    }
                }
            };
            store.dispatch(BillAction::Remove(name_key));
        }
        BillCli::Edit => {
            if store.items().is_empty() {
                println!("No bills can be edited!");
                return Ok(());
            }

            store.print_list();
            let name_key = loop {
                match create_bill_key() {
                    Ok(bill_key) => {
                        if !store.exists(&bill_key) {
                            println!("Bill with key {bill_key:?} does not exist!");
                            continue;
                        }

                        break bill_key;
                    }
                    Err(err) => {
                        if let Some(BillCli::Back) = BillCli::handle_err(&err) {
                            return Ok(());
                        }
                    }
                }
            };

            let new_bill = loop {
                println!("Edit a bill:");

                match create_bill() {
                    Ok(b) => break b,
                    Err(err) => {
                        if let Some(BillCli::Back) = BillCli::handle_err(&err) {
                            return Ok(());
                        }
                    }
                };
            };
            store.dispatch(BillAction::Edit(name_key, new_bill));
        }
        BillCli::List => {
            store.print_list();
        }
        BillCli::Raw => {
            println!("\nStore Items HashMap:\n{store:?}\n\n");
        }
        BillCli::Back => {
            println!("Can't go back from starting point!");
        }
    }
    Ok(())
}

fn handle_edit() {
    // TODO implement this msg
    // if let Some(old_bill) = self.items.get(&key) {
    //     let key_copy = key.clone();
    //     self.items.insert(key, bill);
    //     let new_bill = &self.items.get(&key_copy);
    //     println!(
    //         "Successfully added bill!\n\nOld Bill:\n{:?}New Bill:\n{:?}\n",
    //         old_bill, new_bill
    //     );
    // } else {
    //     println!("Bill with key {key:?} does not exist so it can't be edited!")
    // }
}

fn create_bill() -> Result<Bill, String> {
    println!("1. Enter the bill name:");

    let name = BillCli::get_input()?;
    Bill::validate_name(&name)?;

    println!("2. Enter bill price:");

    let price = BillCli::get_input()?;
    let price = match price.parse::<f64>() {
        Err(err) => {
            return Err(err.to_string());
        }
        Ok(value) => value,
    };
    Bill::validate_price(&price)?;

    Bill::new(name, price)
}

fn create_bill_key() -> Result<String, String> {
    println!("- Enter the bill name to remove:");

    BillCli::get_input()
}

fn main() {
    let mut store = BillStore::create_store();
    let commands = BillCli::list_commands();
    let handle_input = |input: &str| handle_command(input, &mut store);

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
