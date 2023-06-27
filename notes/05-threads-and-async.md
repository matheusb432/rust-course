# Threads and Async Code in Rust 🦀

## Threads

- Threads are a way to run multiple pieces of code at the same time. Rust's standard library provides a thread implementation with safety guarantees.
- Ending the main thread will terminate all spawned threads.

### Spawning a Thread

Creating a new thread is done with the `thread::spawn` function, which takes a closure as an argument. The closure is the code that will be run on the new thread.

```rust
use std::thread;
// Creates a new thread
let handle = thread::spawn(move || {
    println!("Hello from a thread!");
});
// The thread should be joined to the main thread so no data is lost
handle.join();
```
