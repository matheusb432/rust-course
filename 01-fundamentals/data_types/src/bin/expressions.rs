enum Access {
    Admin,
    Manager,
    User,
    Guest,
}

fn main() {
    let access_level = Access::Guest;
    // NOTE Setting the value of can_access_file to the result of the match expression
    let can_access_file = match access_level {
        Access::Admin => true,
        Access::Manager => true,
        _ => false,
    };

    println!("Can access file: {:?}", can_access_file)
}
