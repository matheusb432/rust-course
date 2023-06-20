fn main() {
    let some_bool = true;
    // NOTE match expression, a much more powerful switch statement
    // NOTE match expressions must be exhaustive, they should account for every possible result
    match some_bool {
        true => println!("Is true"),
        false => println!("Is false"),
    }

    let some_int = 5;
    match some_int {
        1 => println!("Is one"),
        2 => println!("Is two"),
        3 => println!("Is three"),
        // NOTE _ is the default case
        _ => println!("Is something else"),
    }

    let my_name = "John";

    match my_name {
        "Jane" => println!("Is Jane"),
        "John" => println!("Is my name"),
        "Bob" => println!("Is Bob"),
        _ => println!("Is someone else"),
    }
}
