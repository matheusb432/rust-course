pub mod constants {
    pub const ADD: &'static str = "add";
    pub const REMOVE: &'static str = "remove";
    pub const EDIT: &'static str = "edit";
    pub const SET_PRICE: &'static str = "set_price";
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
    SetPrice,
    List,
    Back,
    Raw,
}

use crate::{bill::model::Bill, input::get_input};

impl BillCli {
    pub fn new(value: &str) -> Option<Self> {
        let command = match value.to_lowercase().as_str() {
            ADD => BillCli::Add,
            REMOVE => BillCli::Remove,
            EDIT => BillCli::Edit,
            SET_PRICE => BillCli::SetPrice,
            LIST => BillCli::List,
            BACK => BillCli::Back,
            RAW => BillCli::Raw,
            _ => {
                return None;
            }
        };
        Some(command)
    }

    pub fn new_result(value: &str) -> Result<Self, String> {
        match Self::new(value) {
            Some(command) => Ok(command),
            None => Err("Not a valid command!".to_owned()),
        }
    }

    pub fn create_bill() -> Result<Bill, String> {
        println!("1. Enter the bill name:");
        let name = Self::get_name()?;

        println!("2. Enter bill price:");
        let price = Self::get_price()?;

        Bill::new(name, price)
    }

    pub fn key_loop(msg: &str) -> Result<String, ()> {
        Self::bill_input_loop(Self::get_input, Some(msg))
    }

    pub fn new_bill_loop(msg: &str) -> Result<Bill, ()> {
        Self::bill_input_loop(Self::create_bill, Some(msg))
    }

    pub fn list_commands() -> Vec<&'static str> {
        vec![ADD, REMOVE, EDIT, SET_PRICE, LIST, BACK, RAW]
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

    pub fn get_name() -> Result<String, String> {
        Self::get_value(Self::get_name_res)
    }

    pub fn get_price() -> Result<f64, String> {
        Self::get_value(Self::get_price_res)
    }

    fn is_back(input: &str) -> bool {
        input.eq(BACK)
    }

    fn handle_err(err: &str) -> Option<BillCli> {
        if err.eq(GOING_BACK) {
            return Some(BillCli::Back);
        }

        println!("ERROR: {err:?}");
        println!("Trying again..");
        None
    }

    fn get_value<T, F>(getter: F) -> Result<T, String>
    where
        F: Fn() -> Result<T, String>,
    {
        loop {
            match getter() {
                Ok(value) => break Ok(value),
                Err(err) => {
                    if let Some(Self::Back) = Self::handle_err(&err) {
                        return Err(err);
                    }
                }
            }
        }
    }

    fn get_name_res() -> Result<String, String> {
        let name = Self::get_input()?;
        Bill::validate_name(&name)?;
        Ok(name)
    }

    fn get_price_res() -> Result<f64, String> {
        let price = Self::get_input()?;
        let price = match price.parse::<f64>() {
            Err(err) => {
                return Err(err.to_string());
            }
            Ok(value) => value,
        };
        Bill::validate_price(&price)?;
        Ok(price)
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
