use billing_app::bill::{BillAction, BillCli, BillStore};
use billing_app::input::{input_mut_loop, print_help};

fn handle_command(input: &str, store: &mut BillStore) -> Result<(), String> {
    match BillCli::new_result(input)? {
        BillCli::Add => add_bill(store),
        BillCli::Remove => remove_bill(store),
        BillCli::Edit => edit_bill(store),
        BillCli::SetPrice => set_bill_price(store),
        BillCli::List => store.print_list(),
        BillCli::Raw => print_raw(store),
        BillCli::Back => println!("Can't go back from starting point!"),
    }
    Ok(())
}

fn print_raw(store: &BillStore) {
    println!("\nStore Items HashMap:\n{store:?}\n\n");
}

fn get_bill_key(store: &BillStore, msg: &str) -> Option<String> {
    if store.is_empty() {
        println!("No bills found!");
        return None;
    }
    store.print_list();
    // NOTE More concise way to discard the Err of a Result<T>
    BillCli::key_loop(msg).ok()
}

fn add_bill(store: &mut BillStore) {
    let new_bill = match BillCli::new_bill_loop("Add a bill:") {
        Ok(bill) => bill,
        Err(_) => return,
    };
    store.dispatch(BillAction::Add(new_bill));
}

fn remove_bill(store: &mut BillStore) {
    let key = match get_bill_key(&store, "- Enter the bill name to remove:") {
        Some(key) => key,
        None => return,
    };
    store.dispatch(BillAction::Remove(key))
}

fn edit_bill(store: &mut BillStore) {
    let key = match get_bill_key(&store, "- Enter the bill name to edit:") {
        Some(key) => key,
        None => return,
    };
    let new_bill = match BillCli::new_bill_loop("Edit a bill:") {
        Ok(bill) => bill,
        Err(_) => return,
    };

    store.dispatch(BillAction::Edit(key, new_bill));
}

fn set_bill_price(store: &mut BillStore) {
    let key = match get_bill_key(&store, "- Enter the bill name to edit:") {
        Some(key) => key,
        None => return,
    };
    println!("Enter a new price");
    let new_price = match BillCli::get_price() {
        Ok(price) => price,
        Err(_) => return,
    };

    store.dispatch(BillAction::SetPrice(key, new_price));
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
