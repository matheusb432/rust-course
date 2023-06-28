// Topic: Generics & Functions
//
// Requirements:
// * 1. Create a function that accepts the Priority trait as a generic parameter
//   * The function should print out the guest and their priority
// * 2. Use the function with at least two different guests
//
// Notes:
// * Use the debug token "{:?}" to print out the information
// * Use the compiler to guide you to the correct generic constraints needed

#![allow(dead_code)]

use std::fmt::Debug;

#[derive(Debug)]
enum ServicePriority {
    High,
    Standard,
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuest {
    name: String,
}
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug)]
struct Guest {
    name: String,
}
impl Priority for Guest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

// * 1.
// NOTE T is a type that needs to implement both the Priority and Debug traits
fn print_priority<T: Priority + Debug>(priority: &T) {
    println!("{:?} has priority {:?}", priority, priority.get_priority());
}

fn main() {
    let john = Guest {
        name: "John".to_owned(),
    };
    let jake = Guest {
        name: "Jake".to_owned(),
    };
    let jim = ImportantGuest {
        name: "Jim".to_owned(),
    };

    // * 2.
    print_priority(&john);
    print_priority(&jake);
    print_priority(&jim);
}
