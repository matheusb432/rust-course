use std::collections::HashMap;

use super::Bill;

pub enum BillAction {
    Add(Bill),
    Remove(String),
    Edit(String, Bill),
}

type VecStore = Vec<Bill>;
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

    pub fn dispatch(&mut self, action: BillAction) {
        let items = &mut self.items;
        match action {
            BillAction::Add(bill) => {
                let key = bill.key();
                if !items.contains_key(&key) {
                    items.insert(key, bill);
                    println!("Successfully added bill!");
                } else {
                    println!("Bill with key {key:?} already exists!")
                }
            }
            BillAction::Edit(key, bill) => {
                if items.contains_key(&key) {
                    items.insert(key, bill);
                    println!("Successfully edited bill!");
                } else {
                    println!("Bill with key {key:?} does not exist so it can't be edited!")
                }
            }
            BillAction::Remove(name) => {
                let key = &Bill::create_key(&name);

                if items.contains_key(key) {
                    items.remove(&Bill::create_key(&name));
                    println!("Successfully removed bill!");
                } else {
                    println!("Bill with key {key:?} not found!")
                }
            }
        }
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
