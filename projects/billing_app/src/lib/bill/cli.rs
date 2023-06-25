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

pub enum BillCli {
    Add,
    Remove,
    Edit,
    List,
    Back,
    Raw,
}

use crate::{bill::model::Bill, input::get_input};

impl BillCli {
    pub fn create_bill() -> Result<Bill, String> {
        println!("1. Enter the bill name:");

        let name = Self::get_input()?;
        Bill::validate_name(&name)?;

        println!("2. Enter bill price:");

        let price = Self::get_input()?;
        let price = match price.parse::<f64>() {
            Err(err) => {
                return Err(err.to_string());
            }
            Ok(value) => value,
        };
        Bill::validate_price(&price)?;

        Bill::new(name, price)
    }

    pub fn key_loop(msg: &str) -> Result<String, ()> {
        Self::bill_input_loop(Self::get_input, Some(msg))
    }

    pub fn new_bill_loop(msg: &str) -> Result<Bill, ()> {
        Self::bill_input_loop(Self::create_bill, Some(msg))
    }

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

    fn bill_input_loop<T>(
        handle_input: impl Fn() -> Result<T, String>,
        maybe_msg: Option<&str>,
    ) -> Result<T, ()> {
        loop {
            if let Some(msg) = maybe_msg {
                println!("{msg}");
            }

            match handle_input() {
                Ok(bill_key) => break Ok(bill_key),
                Err(err) => {
                    if let Some(BillCli::Back) = BillCli::handle_err(&err) {
                        return Err(());
                    }
                }
            }
        }
    }
}
