// Topic: Multithreading
//
// Requirements:
// * 1. Run the provided functions in threads
// * 2. Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

use std::thread;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(100));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(100));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(100));
    "!"
}

fn main() {
    // ? Also valid syntax
    // let hello = thread::spawn(msg_hello);
    // NOTE the execution of the threads will start and finish in any order
    let hello = thread::spawn(|| {
        println!("Running hello...");
        msg_hello()
    });
    let thread = thread::spawn(|| {
        println!("Running thread...");
        msg_thread()
    });
    let excited = thread::spawn(|| {
        println!("Running excited...");
        msg_excited()
    });

    // ? Different ways to unwrap thread joins without handling the error specifically
    let Ok(hello) = hello.join() else {
        panic!("error in `hello` thread!")
    };
    let thread = thread.join().expect("error in `thread` thread!");
    let excited = excited
        .join()
        .unwrap_or_else(|_| panic!("error in `excited` thread!"));

    // ? A deterministic output since all threads will have finished their execution
    println!("{hello}{thread}{excited}");
}
