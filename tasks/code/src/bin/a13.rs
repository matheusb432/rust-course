// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * 1. Use a vector to store 4 numbers
// * 2. Iterate through the vector using a for..in loop
// * 3. Determine whether to print the number or print "thirty" inside the loop
// * 4. Use the .len() function to print the number of elements in a vector

fn main() {
    // * 1.
    let numbers: Vec<i32> = vec![10, 20, 30, 40];

    // * 2.
    // NOTE: Creating a for loop transfers ownership of the vector, so it's necessary to iterate on &numbers to not move the vector after the loop executes
    for number in &numbers {
        // * 3.
        match number {
            30 => println!("thirty"),
            _ => println!("{number:?}"),
        }
    }

    // * 4.
    println!("Vector length = {:?}", numbers.len());
}
