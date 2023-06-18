// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is greater than 5, less than 5, or equal to 5
//
// Notes:
// * 1. Use a function that returns a tuple
// * 2. Destructure the return value into two variables
// * 3. Use an if..else if..else block to determine what to print

// * 3.
fn display_msg(val: i32) {
    print!("y is {val}, {val} ");
    if val == 5 {
        println!("== 5")
    } else if val > 5 {
        println!("> 5")
    } else {
        println!("< 5")
    }
}

// * 1.
fn get_coordinate() -> (i32, i32) {
    (3, 4)
}

fn main() {
    // * 2.
    let (_, y) = get_coordinate();

    display_msg(y);
    display_msg(10);
    display_msg(5);
}
