// Topic: Iterator
//
// Requirements:
// * 1. Triple the value of each item in a vector.
// * 2. Filter the data to only include values > 10.
// * 3. Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn main() {
    let mapped: Vec<_> = vec![1, 2, 3, 4, 5]
        .iter()
        // * 1.
        .map(|num| num * 3)
        // * 2.
        .filter(|num| num > &10)
        .collect();
    // NOTE joining the iterable manually
    // let last_index = mapped.len() - 1;
    // let output = String::from_iter(mapped.iter().enumerate().map(|(i, num)| {
    //     if i == last_index {
    //         format!("{num:?}")
    //     } else {
    //         format!("{num:?}, ")
    //     }
    // }));

    for num in &mapped {
        // * 3.
        println!("{:?}", &num)
    }

    let mapped_strings: Vec<_> = mapped.iter().map(|num| num.to_string()).collect();
    let output = mapped_strings.join(", ");

    println!("{output:?}");
}
