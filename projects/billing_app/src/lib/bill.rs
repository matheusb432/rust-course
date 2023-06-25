use std::collections::HashMap;

pub mod constants {
    pub const ADD: &'static str = "add";
    pub const REMOVE: &'static str = "remove";
    pub const EDIT: &'static str = "edit";
    pub const LIST: &'static str = "list";
    pub const BACK: &'static str = "/back";
    pub const RAW: &'static str = "raw";

    pub const GOING_BACK: &'static str = "GOING_BACK";
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
    Edit(String, Bill),
}

pub enum BillCli {
    Add,
    Remove,
    Edit,
    List,
    Back,
    Raw,
}

use crate::input::get_input;

impl BillCli {
    pub fn list_commands() -> Vec<&'static str> {
        vec![ADD, REMOVE, EDIT, LIST, BACK, RAW]
    }

    pub fn get_input() -> Result<String, String> {
        match get_input() {
            Ok(input) => {
                if Self::is_back(&input) {
                    println!("Going back...");
                    return Err("GOING_BACK".to_owned());
                }
                Ok(input)
            }
            Err(err) => Err(err.to_string()),
        }
    }

    pub fn is_back(input: &str) -> bool {
        input.eq(BACK)
    }

    pub fn handle_err(err: &str) -> Option<BillCli> {
        if err.eq(GOING_BACK) {
            return Some(BillCli::Back);
        }

        println!("ERROR: {err:?}");
        println!("Trying again..");
        None
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
            RAW => BillCli::Raw,
            _ => {
                return None;
            }
        };
        Some(command)
    }
}
type VecStore = Vec<Bill>;
type BillHashMap = HashMap<String, Bill>;

#[derive(Debug)]
pub struct BillStore {
    // NOTE using BillStore to manage the state encapsulates the items hash map
    items: BillHashMap,
}

impl BillStore {
    // TODO create BillStore struct? To hide implementation details
    pub fn create_store() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn items(&self) -> &BillHashMap {
        &self.items
    }

    pub fn print_list(&self) {
        if self.items.is_empty() {
            println!("\nNo bills registered!");
        } else {
            println!("\nList of bills:\nKey\t|\tName\t|\tPrice");
            for (key, bill) in self.items.iter() {
                print!("{key:?}\t|\t");
                bill.print()
            }
        }
        println!("\n")
    }

    pub fn by_name(&self, name: &str) -> Option<&Bill> {
        let items = &self.items;

        items.get(&Bill::create_key(&name))
    }
    pub fn exists(&self, name: &str) -> bool {
        let items = &self.items;

        items.contains_key(&Bill::create_key(&name))
    }

    pub fn dispatch(
        &mut self,
        action: BillAction,
        // TODO as Result<> & inform err?
    ) {
        let items = &mut self.items;
        match action {
            // TODO add should fail if bill exists?
            BillAction::Add(bill) => {
                let key = bill.key();
                if !items.contains_key(&key) {
                    println!("Successfully added bill!\n\nAdded Bill:\n{:?}\n", &bill);
                    items.insert(key, bill);
                } else {
                    println!("Bill with key {key:?} already exists!")
                }
            }
            BillAction::Edit(key, bill) => {
                if items.contains_key(&key) {
                    items.insert(key, bill);
                } else {
                    println!("Bill with key {key:?} does not exist so it can't be edited!")
                }
            }
            BillAction::Remove(name) => {
                let key = &Bill::create_key(&name);

                if items.contains_key(key) {
                    items.remove(&Bill::create_key(&name));
                    println!("Successfully removed {key:?} bill!");
                } else {
                    println!("Bill with key {key:?} not found!")
                }
            }
        }
    }
}

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

        // NOTE fields are private when out of this module so a Bill will only exist if it's valid
        Ok(Self { name, price })
    }

    pub fn key(&self) -> String {
        Self::create_key(&self.name)
    }

    pub fn create_key(name: &str) -> String {
        name.to_lowercase()
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

    pub fn print(&self) {
        println!("{}\t|\t${}", self.name, self.price)
    }
}
