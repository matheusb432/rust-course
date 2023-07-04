use billing_app::bill::cli::constants::{EDIT_NAME_PROMPT, REMOVE_NAME_PROMPT};
use billing_app::bill::{BillAction, BillCli, BillStore};
use billing_app::input::{input_mut_loop, print_help};

fn handle_command(input: &str, store: &mut BillStore) -> Result<(), String> {
    match BillCli::new_result(input)? {
        BillCli::Add => add_bill(store)?,
        BillCli::Remove => remove_bill(store)?,
        BillCli::Edit => edit_bill(store)?,
        BillCli::SetPrice => set_bill_price(store)?,
        BillCli::List => store.print_list(),
        BillCli::Raw => print_raw(store),
        BillCli::Back => println!("Can't gooooo back from starting point!"),
    }
    Ok(())
}

fn print_raw(store: &BillStore) {
    println!("\nStore Items HashMap:\n{store:?}\n\n");
}

fn bill_name(store: &BillStore, msg: &str) -> Option<String> {
    if store.is_empty() {
        println!("No bills found!");
        return None;
    }
    store.print_list();
    // NOTE More concise way to discard the Err of a Result<T>
    BillCli::key_loop(msg).ok()
}

type ModResult = Result<(), String>;

fn add_bill(store: &mut BillStore) -> ModResult {
    let new_bill = BillCli::new_bill_loop("Add a bill:").map_err(|_| "new bill err")?;
    store.dispatch(BillAction::Add(new_bill));
    Ok(())
}

fn remove_bill(store: &mut BillStore) -> ModResult {
    let key = bill_name(store, REMOVE_NAME_PROMPT).ok_or("no bill key")?;
    store.dispatch(BillAction::Remove(key));
    Ok(())
}

fn edit_bill(store: &mut BillStore) -> ModResult {
    let key = bill_name(store, EDIT_NAME_PROMPT).ok_or("no bill key")?;
    let new_bill = BillCli::new_bill_loop("Edit a bill:").map_err(|_| "new bill err")?;

    store.dispatch(BillAction::Edit(key, new_bill));
    Ok(())
}

fn set_bill_price(store: &mut BillStore) -> ModResult {
    let key = bill_name(store, EDIT_NAME_PROMPT).ok_or("no bill key")?;
    println!("Enter a new price");
    let new_price = BillCli::get_price().map_err(|_| "err on bill price")?;

    store.dispatch(BillAction::SetPrice(key, new_price));
    Ok(())
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
