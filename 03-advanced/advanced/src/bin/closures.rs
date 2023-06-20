fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let sum = add_fn(1, 1);

    // NOTE creating a closure for the function with pipes `| |`
    // let add = |a: i32, b: i32| -> i32 { a + b };
    // NOTE Rust can infer the type of this closure params and return by it's usage in sum_2
    let add = |a, b| a + b;
    let sum_2 = add(1, 1);

    println!("sum: {}\nsum_2: {}", sum, sum_2);
}
