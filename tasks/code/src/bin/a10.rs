// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * 1. Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * 2. Use a function to print the messages
// * 3. Use a match expression to determine which message
//   to print

// * 2.
fn print_msg(val: bool) {
    // * 3.
    match val {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    let value = 99;
    let is_gt_100 = value > 100;
    print_msg(is_gt_100);

    // ? More concise solution
    let res_text = if value > 100 { "its big" } else { "its small" };

    println!("res_text -> {res_text:?}")
}
