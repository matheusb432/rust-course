// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * 1. Use an enum for the tickets with data associated with each variant
// * 2. Create one of each ticket and place into a vector
// * 3. Use a match expression while iterating the vector to print the ticket info

// * 1.
enum Ticket {
    Standard(f32),
    Backstage(f32, String),
    Vip(f32, String),
}

fn main() {
    // * 2.
    let standard = Ticket::Standard(49.99);
    let backstage = Ticket::Backstage(99.99, "Jake Weary".to_owned());
    let backstage_2 = Ticket::Backstage(129.99, "Etc".to_owned());
    let vip = Ticket::Vip(149.99, "John Rust".to_owned());

    let tickets = vec![standard, backstage, vip, backstage_2];

    for ticket in &tickets {
        match ticket {
            Ticket::Standard(price) => println!("Standard ticket price -> {:?}", price),
            Ticket::Backstage(price, holder) => println!(
                "Backstage✨ ticket price -> {:?} of holder {:?}",
                price, holder
            ),
            Ticket::Vip(price, holder) => {
                println!("Vip🌟 ticket price -> {:?} of holder {:?}", price, holder)
            }
        }
    }
}
