// Topic: Macro practice
//
// Summary:
//   Create a macro that measures how long a function takes to execute.
//
// Requirements:
// * 1. Write a single macro that executes a function:
//   * Prior to executing the function, print out "Call: ", followed
//     by the function name
//   * Measure how long the function takes to executes
//   * Print out (in nanoseconds) how long the function takes to execute
// * 2. Measure each sample function with the macro
//
// Notes:
// * `std::time::Instant` can be used to calculate elapsed time
// * Use `stringify!` to get a string representation of the function name

// ? 1.
macro_rules! measure_fn {
    ($fn: ident $(=>)? $( $args:expr),*) => {{
        // NOTE Defining an import in a macro will also import it in the calling scope
        // use ::std::time::Instant;

        let fn_name = stringify!($fn);
        println!("Call: {fn_name}");
        let start = ::std::time::Instant::now();
        // NOTE dynamically calling function with any number of arguments
        $fn($($args,)*);
        let end = ::std::time::Instant::now();
        println!(
            "Execution time of {fn_name}: {:?}ns",
            end.duration_since(start).as_nanos()
        );
    }};
}

macro_rules! fnlog {
    // * With token tree
    ($fn: ident $( $args:tt)* ) => {
        let fn_name = stringify!($fn);
        println!("Call: {fn_name}");
        let start = ::std::time::Instant::now();
        $fn$($args)*;
        let end = ::std::time::Instant::now();
        println!(
            "Execution time of {fn_name}: {:?}ns",
            end.duration_since(start).as_nanos()
        );
    };
}

fn sample_fn_1() {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(2));
}
fn sample_fn_2(n: u64) {
    let mut n = n;
    while n > 0 {
        use std::time::Duration;
        std::thread::sleep(Duration::from_micros(n));
        n -= 1;
    }
}
fn sample_fn_3(lhs: usize, rhs: usize) -> usize {
    lhs + rhs
}

fn main() {
    // ? 2.
    measure_fn!(sample_fn_1);
    measure_fn!(sample_fn_2 => 5 + 10);
    fnlog!(sample_fn_2(5 + 10));
    measure_fn!(sample_fn_3 => 10, 20);
    fnlog!(sample_fn_3(10, 20));
}
