use billing_app::bill::{BillAction, BillCli, BillStore};
use billing_app::input::{input_mut_loop, print_help};

fn handle_command(input: &str, store: &mut BillStore) -> Result<(), String> {
    let cli_command = BillCli::new_result(input)?;

    match cli_command {
        BillCli::Add => add_bill(store),
        BillCli::Remove => remove_bill(store),
        BillCli::Edit => edit_bill(store),
        BillCli::List => store.print_list(),
        BillCli::Raw => print_raw(store),
        BillCli::Back => println!("Can't go back from starting point!"),
    }
    Ok(())
}

fn print_raw(store: &BillStore) {
    println!("\nStore Items HashMap:\n{store:?}\n\n");
}

fn add_bill(store: &mut BillStore) {
    let new_bill = match BillCli::new_bill_loop("Add a bill:") {
        Ok(bill) => bill,
        Err(_) => return,
    };
    store.dispatch(BillAction::Add(new_bill));
}

fn remove_bill(store: &mut BillStore) {
    if store.is_empty() {
        println!("No bills can be removed!");
        return;
    }

    store.print_list();
    if let Ok(key) = BillCli::key_loop("- Enter the bill name to remove:") {
        store.dispatch(BillAction::Remove(key))
    }
}

fn edit_bill(store: &mut BillStore) {
    if store.is_empty() {
        println!("No bills can be edited!");
        return;
    }

    store.print_list();

    let key = match BillCli::key_loop("- Enter the bill name to edit:") {
        Ok(key) => key,
        Err(_) => return,
    };
    let new_bill = match BillCli::new_bill_loop("Edit a bill:") {
        Ok(bill) => bill,
        Err(_) => return,
    };

    store.dispatch(BillAction::Edit(key, new_bill));
}

fn main() {
    let mut store = BillStore::new();
    let commands = BillCli::list_commands();
    let handle_input = |input: &str| handle_command(input, &mut store);

    print_help(&commands);
    match input_mut_loop(handle_input, &commands) {
        Ok(_) => println!("Exiting program..."),
        Err(err) => println!("ERROR: {err:?}"),
    }
}
