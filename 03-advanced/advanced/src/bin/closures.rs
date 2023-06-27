fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

// NOTE Passing a closure as a trait object function parameter in a Box<> on the heap
fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    let sum = add_fn(1, 1);

    // NOTE creating a closure for the function with pipes `| |`
    // let add = |a: i32, b: i32| -> i32 { a + b };
    // NOTE Rust can infer the type of this closure params and return by it's usage in sum_2
    let add = |a, b| a + b;
    let sum_2 = add(1, 1);

    println!("sum: {}\nsum_2: {}", sum, sum_2);

    let add = |a: i32, b: i32| a + b;
    // NOTE Creating a box with a closure
    let add = Box::new(add);
    math(1, 2, add);
}
