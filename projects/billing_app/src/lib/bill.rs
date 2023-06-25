// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.

use std::collections::HashMap;

pub mod constants {
    // * CLI Commands
    pub const ADD: &'static str = "add";
    pub const REMOVE: &'static str = "remove";
    pub const EDIT: &'static str = "edit";
    pub const LIST: &'static str = "list";
    pub const BACK: &'static str = "back";
}
use constants::*;

// TODO split into bill/store & bill/commands modules?
#[derive(Debug)]
pub struct Bill {
    name: String,
    price: f64,
}

pub enum BillAction {
    Add(Bill),
    Remove(String),
    Edit(Bill),
}

pub enum BillCli {
    Add,
    Remove,
    Edit,
    List,
    Back,
}

impl BillCli {
    pub fn list_commands() -> Vec<&'static str> {
        vec![ADD, REMOVE, EDIT, LIST, BACK]
    }

    pub fn new_result(value: &str) -> Result<Self, String> {
        match Self::new(value) {
            Some(command) => Ok(command),
            None => Err("Not a valid command!".to_owned()),
        }
    }

    pub fn new(value: &str) -> Option<Self> {
        let command = match value.to_lowercase().as_str() {
            ADD => BillCli::Add,
            REMOVE => BillCli::Remove,
            EDIT => BillCli::Edit,
            LIST => BillCli::List,
            BACK => BillCli::Back,
            _ => {
                return None;
            }
        };
        Some(command)
    }
}

type VecStore = Vec<Bill>;

// TODO Find a way to make the bill store readonly so that only actions can be dispatched from the outside
// * Currently the consumer of this store could just do anything with it
impl Bill {
    pub fn validate_name(name: &str) -> Result<(), String> {
        if name.is_empty() {
            Err("Name cannot be empty!".to_owned())
        } else {
            Ok(())
        }
    }

    pub fn validate_price(price: &f64) -> Result<(), String> {
        if !price.is_sign_positive() {
            Err("Price must be greater than 0!".to_owned())
        } else {
            Ok(())
        }
    }

    pub fn new(name: String, price: f64) -> Result<Self, String> {
        Self::validate_name(name.as_str())?;
        Self::validate_price(&price)?;

        Ok(Self { name, price })
        // TODO clean
        // if name.len() == 0 {
        //     Err("Name cannot be empty!".to_owned())
        // } else if price < 0.0 {
        //     Err("Price must be greater than 0!".to_owned())
        // } else {
        //     Ok(Self { name, price })
        // }
    }

    // TODO create BillStore struct? To hide implementation details
    pub fn create_store() -> HashMap<String, Self> {
        HashMap::new()
    }

    pub fn dispatch(
        state: &mut HashMap<String, Self>,
        action: BillAction,
        // TODO as Result<> & inform err?
    ) -> &HashMap<String, Self> {
        match action {
            BillAction::Add(bill) => {
                let key = bill.name.to_lowercase();

                state.insert(key, bill);
            }
            BillAction::Edit(bill) => {
                let key = bill.name.to_lowercase();

                state.insert(key, bill);
            }
            BillAction::Remove(name) => {
                state.remove(&name.to_lowercase());
            }
        }
        state
    }

    pub fn create_vec_store() -> VecStore {
        vec![]
    }

    pub fn dispatch_vec(state: &mut VecStore, action: BillAction) -> &VecStore {
        match action {
            BillAction::Add(new_bill) => {
                println!("Successfully added bill! Added Bill:\n{:?}", &new_bill);
                state.push(new_bill);
            }
            _ => panic!("Not implemented!"),
        }
        state
    }
}
