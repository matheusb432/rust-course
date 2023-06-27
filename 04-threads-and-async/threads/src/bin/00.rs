use std::thread;

fn main() {
    let iterations = 10;
    // NOTE The output is non deterministic, a "B" iteration may run while "A" is running
    let thread_a = thread::spawn(move || {
        for i in 1..=iterations {
            println!("A:{i}");
        }
    });
    let thread_b = thread::spawn(move || {
        for i in 1..=iterations {
            println!("\tB:{i}");
        }
    });

    // NOTE Without joining the threads, the main thread can terminate before threads A and B finish their execution
    thread_a.join();
    thread_b.join();
}
