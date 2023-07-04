use std::collections::{hash_map::Entry, HashMap};

use super::Bill;

pub enum BillAction {
    Add(Bill),
    Remove(String),
    Edit(String, Bill),
    SetPrice(String, f64),
}

type BillHashMap = HashMap<String, Bill>;

#[derive(Debug)]
pub struct BillStore {
    // NOTE using BillStore to manage the state encapsulates the items hash map
    items: BillHashMap,
}

impl BillStore {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn get_all(&self) -> Vec<&Bill> {
        self.items.values().collect()
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

        items.get(&Bill::create_key(name))
    }

    pub fn exists(&self, name: &str) -> bool {
        let items = &self.items;

        items.contains_key(&Bill::create_key(name))
    }

    pub fn dispatch(&mut self, action: BillAction) {
        use BillAction::*;

        match action {
            Add(bill) => {
                let key = bill.key();
                // NOTE Entry::Vacant & Entry::Occupied are enums that represent the state of a HashMap entry
                // ? They can be used as an alternative to .contains_key() and .get_mut()
                if let Entry::Vacant(entry) = self.items.entry(key.clone()) {
                    entry.insert(bill);
                    println!("Successfully added bill!");
                } else {
                    println!("Bill with key {key:?} already exists!")
                }
            }
            Edit(key, bill) => {
                if let Entry::Occupied(mut entry) = self.items.entry(key.clone()) {
                    entry.insert(bill);
                    println!("Successfully edited bill!");
                } else {
                    println!("Bill with key {key:?} does not exist so it can't be edited!")
                }
            }
            SetPrice(key, price) => {
                // NOTE get_mut returns a mutable reference in a HashMap by it's key
                if let Some(old_bill) = self.items.get_mut(&key) {
                    match old_bill.set_price(price) {
                        Ok(_) => println!("Successfully set a new price!"),
                        Err(err) => println!("Error setting price: {err}"),
                    }
                }
            }
            Remove(name) => {
                let key = &Bill::create_key(&name);

                // NOTE .is_some() will return true if an item was removed
                if self.items.remove(&Bill::create_key(&name)).is_some() {
                    println!("Successfully removed bill!");
                } else {
                    println!("Bill with key {key:?} not found!")
                }
            }
        }
    }
}

impl Default for BillStore {
    fn default() -> Self {
        Self::new()
    }
}
