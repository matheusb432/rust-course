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

## Channels

- Channels enable one-way communication between threads through message passing.
- Messages are composed of `Send` and `Receive` ends, ends can be cloned and sent to threads.
- Can have limited or unlimited capacity.
- The stdlib provides a channel, but third-party implementations such as the `crossbeam-channel` crate have more features and generally better performance.

### Message Passing

- Messages that are sent through channels are usually encapsulated in an `enum`, but any data can be sent.
- Guaranteed in-order delivery of messages, which enables deterministic behavior.
- Message passing can be blocking or non-blocking.
  - blocking: the sender will wait until the receiver is ready to receive the message.
  - non-blocking: the sender will not wait for the receiver to receive the message.
  - Block on sender: channel is full.
  - Block on receiver: channel is empty.
  - The behavior is determined by function, not the channel itself.
