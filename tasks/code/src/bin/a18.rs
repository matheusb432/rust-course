// Topic: Result
//
// Requirements:
// * 1. Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * 2. Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * 3. Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * 4. Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

// * 1.
#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

impl Adult {
    // * 2.
    fn new(name: &str, age: u8) -> Result<Self, &str> {
        if age >= 21 {
            return Ok(Self {
                name: name.to_string(),
                age,
            });
        }

        Err("Age must be over 21")
    }
}

fn main() {
    // * 3.
    let adult = Adult::new("Adult", 22);
    let underage = Adult::new("Not adult", 20);

    // * 4.
    for item in [&adult, &underage] {
        match item {
            Ok(data) => println!("{:?}", data),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
