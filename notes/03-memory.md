# Memory in Rust 🦀

- All programs must track their memory usage. This is done by allocating and freeing memory. If they fail to do so, a memory leak occurs. This is a serious problem, as it can cause a program or even an entire system to crash.
- To transfer access to data without copying it, Rust uses pointers. A pointer is a value that describes a location in memory.
- Rust follows the `Pointer Safety Principle`; data should never be aliased and mutated at the same time. This is enforced at compile time.

## Variables

- Variables live in the stack in memory, they're stored in _frames_. A _frame_ is a mapping from variables to values within a single scope, such as a function.
- Frames are organized into a stack of currently-called-functions. After a function returns, Rust deallocates the function's frame. This sequence of frames is called a stack because the most recent frame added is always the next frame freed.

## Ownership

- Rust utilizes an _ownership_ model to manage memory. This means that every value has a single owner, and that owner is responsible for cleaning up the memory.
- _Ownership_ is a core feature in Rust that ensures memory safety without the need for a garbage collector.
- _Ownership_ is used to keep programs running fast and free from memory leaks. It's much more efficient to borrow data than it is to copy it.
- Each value in Rust has a variable that is its owner. The scope in which the variable is declared is where it is valid or accessible. Once the variable goes out of scope, Rust automatically calls the drop function and the memory is freed.
- In Rust, memory can either be _moved_ or _borrowed_.

### Moving memory

- When a value is _moved_, the ownership is transferred to the new owner. the new owner is now responsible for deleting the data

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

- When a value is _borrowed_, the ownership is not transferred, but the borrower is allowed to use the value for a limited time, and it will not take responsibility for deleting the data.
- Creating a reference to mutable data ("borrowing" it) causes that data to be temporarily read-only until the reference is no longer used. This ensures that mutable references are memory safe.
- Mutable references are also called "unique references"
- As a part of the _Pointer Safety Principle_, the borrow checker enforces that data must outlive any references to it.

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

## The Stack and the Heap

- Data is stored in memory in two principal areas: the stack and the heap.

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
- Lifetimes are a form of generics, by convention they're named with `'a, 'b, 'c`.

  - 'static data lives for the entire duration of the program

- Lifetimes annotations indicates that there exists some owned data that:
  - "Lives at least as long" as the borrowed data
  - "Outlives or outlasts" the scope of a borrow
  - "Exists longer than" the scope of a borrow
- Structs using borrowed data must:
  - Always be created _after_ the owner was created
  - Always be destroyed _before_ the owner is destroyed

```rust
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

- Lifetime annotations don’t change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.

- In this example We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### Lifetime Elision

- Rust has a set of rules for implicit lifetime annotations that are known as the _lifetime elision rules_. These aren't rules for programmers to follow; they're a set of particular cases that the compiler will consider, and if your code fits these cases, you don't need to write the lifetimes explicitly.

- The compiler uses _three rules_ to figure out what lifetimes references have when there aren't explicit annotations. If the compiler gets to the end of the three rules and there are still references for which it can't figure out lifetimes, the compiler will stop with an error.

  1. Each parameter that is a reference gets its own lifetime parameter. In other words, a function with one parameter gets one lifetime parameter: `fn foo<'a>(x: &'a i32);` a function with two parameters gets two separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32);` and so on.

  2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`.

  3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of `self` is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.
