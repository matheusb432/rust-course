# Memory in Rust 🦀

All programs must track their memory usage. This is done by allocating and freeing memory. If they fail to do so, a memory leak occurs. This is a serious problem, as it can cause the program or even the entire system to crash.

To transfer access to data without copying it, Rust uses pointers. A pointer is a value that describes a location in memory.

Rust follows the Pointer Safety Principle; data should never be aliased and mutated at the same time. This is enforced at compile time.

## Variables

Variables live in the stack in memory, they're stored in frames. A frame is a mapping from variables to values within a single scope, such as a function.

Frames are organized into a stack of currently-called-functions. After a function returns, Rust deallocates the function's frame. This sequence of frames is called a stack because the most recent frame added is always the next frame freed.

## Ownership

Rust utilizes an "ownership" model to manage memory. This means that every value has a single owner, and that owner is responsible for cleaning up the memory.

Ownership is a core feature in Rust that ensures memory safety without the need for a garbage collector.

Ownership is used to keep programs running fast and free from memory leaks. It's much more efficient to borrow data than it is to copy it.

Each value in Rust has a variable that is its owner. The scope in which the variable is declared is where it is valid or accessible. Once the variable goes out of scope, Rust automatically calls the drop function and the memory is freed.

In Rust, memory can either be "moved" or "borrowed".

### Moving memory

When a value is moved, the ownership is transferred to the new owner. the new owner is now responsible for deleting the data

```rust
enum Light {
    Bright,
    Dull,
}

// Any function that owns data is required to delete it once it completes, this means that light will be deleted in memory once the function finishes
fn display_light(light: Light) {
    match light {
        Light::Bright => println!("Bright"),
        Light::Dull => println!("Dull"),
    }
}

// Dull is owned by the main function
let dull = Light::Dull;
// Passing dull as an argument will "move" it to the display_light function
display_light(dull);
// This will NOT compile, dull has already been deleted in memory
display_light(dull);
```

### Borrowing memory

When a value is borrowed, the ownership is not transferred, but the borrower is allowed to use the value for a limited time, and it will not take responsibility for deleting the data.

```rust
enum Light {
    Bright,
    Dull,
}

// The `&` symbol indicates that the function is borrowing data, rather than taking ownership of it
fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("Bright"),
        Light::Dull => println!("Dull"),
    }
}

let dull = Light::Dull;
// Passing `&dull` will "borrow" it to the display_light function
display_light(&dull);
// This works since dull has not yet been deleted in memory
display_light(&dull);
```

Creating a reference to mutable data ("borrowing" it) causes that data to be temporarily read-only until the reference is no longer used. This ensures that mutable references are memory safe.

Mutable references are also called "unique references"

As a part of the Pointer Safety Principle, the borrow checker enforces that data must outlive any references to it.

## The Stack and the Heap

In Rust, data is stored in memory in two principal areas: the stack and the heap.

### Stack

- The stack is an area of memory that stores values in the order in which they are received and removes them in the opposite order. This behavior is known as last-in, first-out (LIFO).
- Data stored on the stack must have a known, fixed size at compile time.
- Accessing data on the stack is typically faster than accessing data on the heap because the stack is structured in such a way that it uses offsets to access data quickly.
- In Rust, basic data types like integers, floating-point numbers, and structs with fixed size are typically stored on the stack.
- Vectors in Rust use both the stack and the heap. The vector's metadata (such as length, capacity, and a pointer to the data) is stored on the stack, while the actual elements of the vector are stored on the heap.
- The fact that vectors have a single type of data is related to Rust's static typing and not directly linked to their use of the stack for metadata. Rust ensures that elements of a vector are of the same type for type safety and performance reasons.

### Heap

- The heap is an area of memory where data is allocated dynamically, at runtime.
- A pointer to the heap-allocated data is usually stored on the stack. This pointer keeps the memory address of where the data is actually stored on the heap.
- Accessing data on the heap is generally slower than on the stack because the memory address must be dereferenced, and the heap has more complex memory management.
- In Rust, data types like vectors, strings, and other dynamically-sized collections are stored on the heap because they can grow or shrink as needed.

```rust
struct Entry {
    id: i32,
}

let data = Entry { id: 5 };
// NOTE Putting the data on the heap, Box is a pointer to some data on the heap
let data_ptr: Box<Entry> = Box::new(data);
// NOTE Dereferencing the pointer to move the data back to the Stack
let data_stack = *data_ptr;
```

## Lifetimes

- Lifetimes are a way to ensure that references are valid as long as the data they refer to is valid.
- They're needed to store borrowed data in structs or enums, and to return borrowed data from functions.
- All data has a lifetime, but the compiler can often infer the lifetime of data, so most times it's unnecessary to explicitly annotate lifetimes.
- Lifetimes are a form of generics, by convention they're named with 'a, 'b, 'c, etc.

  - 'static data lives for the entire duration of the program

- Lifetimes annotations indicates that there exists some owned data that:
  - "Lives at least as long" as the borrowed data
  - "Outlives or outlasts" the scope of a borrow
  - "Exists longer than" the scope of a borrow
- Structs using borrowed data must:
  - Always be created _after_ the owner was created
  - Always be destroyed _before_ the owner is destroyed

```rust
// Syntax
fn name<'a>(arg: &'a DataType) -> &'a DataType {}

enum Part {
    Bolt,
    Panel,
}

// A lifetime called 'a is declared
struct RobotArm<'a> {
    // Part` will be created before `RobotArm` and will live at least as long as `RobotArm`
    part: &'a Part,
}

struct AssemblyLine {
    parts: Vec<Part>,
}

let line = AssemblyLine {
    parts: vec![Part::Bolt, Part::Panel],
};

{
    let arm = RobotArm {
        // The lifetime of the reference is the same as the lifetime of the AssemblyLine
        // * BUT the part itself is not owned by RobotArm, so it won't be deleted when RobotArm is dropped
        part: &line.parts[0],
    };
}
```

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
