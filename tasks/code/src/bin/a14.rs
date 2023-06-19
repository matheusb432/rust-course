// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * 1. Use a struct for a persons age, name, and favorite color
// * 2. The color and name should be stored as a String
// * 3. Create and store at least 3 people in a vector
// * 4. Iterate through the vector using a for..in loop
// * 5. Use an if expression to determine which person's info should be printed
// * 6. The name and colors should be printed using a function

// * 1. 2.
struct Person {
    age: u8,
    name: String,
    favorite_color: String,
}

// * 6.
fn print_info(person: &Person) {
    println!("For {}:", person.name);

    print_str(&person.name);
    print_str(&person.favorite_color);
}

fn print_str(value: &str) {
    println!("\t{value:?}")
}

fn main() {
    // * 3.
    let people = vec![
        Person {
            age: 10,
            name: String::from("Alice"),
            favorite_color: String::from("blue"),
        },
        Person {
            age: 11,
            name: String::from("Bob"),
            favorite_color: String::from("red"),
        },
        Person {
            age: 5,
            name: String::from("Carol"),
            favorite_color: String::from("green"),
        },
    ];

    // * 4.
    for person in &people {
        // * 5.
        if person.age <= 10 {
            print_info(person);
        }
    }
}
