// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * 1. Use an enum to create different flavors of drinks
// * 2. Use a struct to store drink flavor and fluid ounce information
// * 3. Use a function to print out the drink flavor and ounces
// * 4. Use a match expression to print the drink flavor
//
// ? I'm not going to use ounces bc I'm not american >:)

// * 1.
enum Flavor {
    Orange,
    Cola,
}

// * 2.
struct Drink {
    flavor: Flavor,
    fluid_ml: u32,
}

// * 3.
fn print_info(drink: Drink) {
    print!("{:?}ml ", drink.fluid_ml);

    // * 4.
    match drink.flavor {
        Flavor::Orange => println!("Orange soder"),
        Flavor::Cola => println!("Cola soder"),
    }
}

fn main() {
    let fanta = Drink {
        flavor: Flavor::Orange,
        fluid_ml: 350,
    };
    let coke = Drink {
        flavor: Flavor::Cola,
        fluid_ml: 500,
    };

    print_info(fanta);
    print_info(coke);
}
