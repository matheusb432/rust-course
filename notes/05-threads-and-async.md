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

## Shared Ownership

- Shared ownership is a way to have multiple owners of the same data.
- It's useful when you want to share data between multiple parts of your program, but you don't want to have to worry about lifetimes.

### Smart Pointers

- Smart pointers are data structures that behave like pointers, but have additional metadata and capabilities.
- They're useful for implementing shared ownership.
- Reference counted pointers (`Rc`) are a type of smart pointer that keep track of the number of references to a piece of data in single-threaded programs.
- Atomic reference counted pointers (`Arc`) are essentially `Rc` pointers that can be used in multi-threaded programs.
- The data for these pointers is only dropped when all the owners have been dropped.

### Interior Mutability

- Interior mutability is a way to create permantenly mutable data. It has trade-offs in complexity and performance.
- With `Cell`, accessing data results in a move or copy. Data must be copyable and ideally should be cheap to move (such as numbers or booleans).
- With `RefCell`, accessing data results in a borrow. It has efficient data access when compared to `Cell`.
  - It's borrow checked at runtime, so it can cause panics.
- Both `Cell` and `RefCell` are NOT thread safe.
- In general, `mut` and `&mut` is preferred over interior mutability.

### Sharing data with multiple threads

- To safely share data between threads, Synchronization Primitives are used.
- A common primitive is a `Mutex` (`Mut`ually `Ex`clusive lock). It's a smart pointer that uses atomic operations to ensure only one thread accesses data at a time.
  - It's a blocking primitive, meaning that if a thread tries to access data that's locked, it will wait until the data is unlocked.
  - Mutexes cannot be shared between threads, so they're often wrapped in an `Arc` pointer.
  - The stdlib provides a mutex, but the `parking_lot` crate provides a faster mutex.
  - Mutexes can cause deadlocks and performance issues, so CPU heavy tasks should be avoided while holding a lock, or done so before locking.
  - Acquiring a lock is done with the `lock()` method, and unlocking occurs when the lock is dropped.

### Deadlocks

- Deadlock is a situation where locks are waiting on one another, preventing any of them from being released.
- They occur when using multiple locks, recursive locks, or locking the same lock multiple times.

```rust
use parking_lot::Mutex;

fn recurse(
    data: Rc<Mutex<u32>>,
    remaining: usize,
) -> usize {
    // Will lock an already locked mutex in the second call, causing a deadlock
    let mut locked = data.lock();
    match remaining {
        rem if rem == 0 => 0,
        rem => recurse(Rc::clone(&data), rem - 1),
    }
}

// To fix it, a ReentrantMutex can be used, now a thread can lock the same mutex multiple times
use parking_lot::ReentrantMutex;

fn recurse(
    data: Rc<ReentrantMutex<u32>>,
    remaining: usize,
) -> usize {
    // Locking again will only increase the lock count, which will unlock when the lock count reaches 0
    let mut locked = data.lock();
    match remaining {
        rem if rem == 0 => 0,
        rem => recurse(Rc::clone(&data), rem - 1),
    }
}
```

#### Thread Contentions

- Contention is when multiple threads are trying to access the same data at the same time.
- It can cause performance issues, so it's important to avoid it.
- One technique to avoid contention is to use exponential backoff.
  - When a thread tries to access data that's locked, it will wait for a random amount of time before trying again.
  - If it fails again, it will wait for a longer amount of time before trying again.
  - This continues until the thread is able to access the data.
