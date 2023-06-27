use std::{
    thread::{self, JoinHandle},
    time::Duration,
};

fn main() {
    let value: JoinHandle<usize> = thread::spawn(move || {
        // NOTE Async execution
        thread::sleep(Duration::from_secs(1));
        println!("Done waiting!");
        42
    });

    println!("Waiting on thread");

    // NOTE join will return a Result<T, E> with the return of the thread execution
    match value.join() {
        Ok(n) => println!("value: {n}"),
        Err(e) => println!("error joining thread: {e:?}"),
    }
}
