#![allow(dead_code, unused_imports, unused_variables)]

use std::str::FromStr;
use strum::{EnumIter, EnumString, EnumVariantNames};

fn main() {
    // NOTE Serialize string to enum
    #[derive(Debug, EnumString)]
    enum Status {
        // NOTE Configuring multiple possible string representations
        #[strum(serialize = "i", serialize = "Idle")]
        Idle,
        #[strum(serialize = "p")]
        Processing,
    }
    let idle = Status::from_str("i"); // Ok(Idle)
    println!("{:?}", idle);
    let processing = Status::from_str("p"); // Ok(Processing)
    println!("{:?}", processing);
    let processing = Status::from_str("Processing"); // Err(VariantNotFound)
    println!("{:?}", processing);
}
