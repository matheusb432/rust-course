enum Discount {
    Percent(i32),
    Flat(i32),
}

struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 5;
    match n {
        3 => println!("three"),
        // NOTE It's possible to name the catch-all case, `other` in this case is `n`
        other => println!("number: {:?}", other),
    }

    // NOTE Creating an enum variation
    let flat = Discount::Flat(2);

    match flat {
        Discount::Flat(2) => println!("Flat 2"),
        // NOTE Discount::Flat(amount) is the catch-all case
        Discount::Flat(amount) => println!("Flat discount of {:?}", amount),
        // NOTE _ => () indicates a return of nothing
        _ => (),
    }

    let concert = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };

    match concert {
        // NOTE Matching only when price is 50
        Ticket { price: 50, event } => println!("event @ 50 = {:?}", event),
        // NOTE the `..` serves to ignore other fields
        Ticket { price, .. } => println!("price = {:?}", price),
    }
}
