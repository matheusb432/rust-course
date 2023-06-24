// Topic: External Modules
//
// Summary:
// The existing program is complete, but all the code exists
// in a single module. This code can benefit from being organized
// into multiple external modules.
//
// Requirements:
// * Organize the code into two external modules based on their functionality:
//   - msg: string formatting functions
//   - math: math functions
// * Update the main function to use the functionality from the modules
//
// Notes:
// * Update your Cargo.toml to include a library file
// * After moving the functions into modules, try running
//   `cargo check --bin a26c` to get a listing of required code changes

fn main() {
    use code::math;

    let result = {
        // use code::math::{add, mul, sub};
        // NOTE the shorthand is allowed here because the `math` module is in scope
        use math::{add, mul, sub};

        let two_plus_two = add(2, 2);
        let three = sub(two_plus_two, 1);
        mul(three, three)
    };

    assert_eq!(result, 9);
    println!("(2 + 2 - 1) * 3 = {}", result);

    use code::msg::{capitalize, exciting, trim};

    let hello = {
        let msg = "hello ";
        let msg = trim(msg);
        capitalize(msg)
    };
    let world = {
        let msg = "world";
        exciting(msg)
    };
    let msg = format!("{}, {}", hello, world);

    assert_eq!(&msg, "Hello, world!!!");
    println!("{}", msg);
}
