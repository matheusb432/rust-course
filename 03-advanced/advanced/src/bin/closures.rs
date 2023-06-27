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
    // * This is necessary to call the closure from a function, as a Box has a fixed size (the
    // * size of a pointer) and the compiler can't know the size of the closure
    let add = Box::new(add);
    let sub = Box::new(|a, b| a - b);
    let mul = Box::new(|a, b| {
        println!("Multiplying {} and {}", a, b);
        a * b
    });
    let name = "some name".to_owned();
    // NOTE `move` keyword is necessary to move the ownership of the variable to the closure
    let div = Box::new(move |a, b| {
        println!("name: {:?}", name);
        a / b
    });
    // * This won't compile as `name` has been moved to the closure
    // * Can be fixed by using only a &str in the closure
    // println!("name: {:?}", name);

    math(1, 2, add);
    math(2, 3, sub);
    let res = math(3, 4, mul);
    println!("res: {}", res);
    let res = math(8, 4, div);
    println!("res: {}", res);
}
