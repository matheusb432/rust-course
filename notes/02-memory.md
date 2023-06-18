# Memory

All programs must track their memory usage. This is done by allocating and freeing memory. If they fail to do so, a memory leak occurs. This is a serious problem, as it can cause the program or even the entire system to crash.

## Ownership

Rust utilizes an "ownership" model to manage memory. This means that every value has a single owner, and that owner is responsible for cleaning up the memory.

Ownership is a core feature in Rust that ensures memory safety without the need for a garbage collector.

Ownership is used to keep programs running fast and free from memory leaks. It's much more efficient to borrow data than it is to copy it.

Each value in Rust has a variable that is its owner. The scope in which the variable is declared is where it is valid or accessible. Once the variable goes out of scope, Rust automatically calls the drop function and the memory is freed.

In Rust, memory can either be "moved" or "borrowed".

When a value is moved, the ownership is transferred to the new owner. the new owner is now responsible for deleting the data

Moving memory:

```rust
enum Light {
    Bright,
    Dull,
}

// NOTE any function that owns data is required to delete it once it completes, this means that light will be deleted in memory once the function finishes
fn display_light(light: Light) {
    match light {
        Light::Bright => println!("Bright"),
        Light::Dull => println!("Dull"),
    }
}

fn main() {
    // NOTE dull is owned by the main function
    let dull = Light::Dull;
    // NOTE passing dull as an argument will "move" it to the display_light function
    display_light(dull);
    // NOTE This will NOT compile, dull has already been deleted in memory
    display_light(dull);
}
```

When a value is borrowed, the ownership is not transferred, but the borrower is only allowed to use the value for a limited time, and it will not take responsibility for deleting the data.

Borrowing memory:

```rust


enum Light {
    Bright,
    Dull,
}

// NOTE the `&` symbol indicates that the function is borrowing data, rather than taking ownership of it
fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("Bright"),
        Light::Dull => println!("Dull"),
    }
}

fn main() {
    let dull = Light::Dull;
    // NOTE passing `&dull` will "borrow" it to the display_light function
    display_light(&dull);
    // NOTE This will work now since dull has not yet been deleted in memory
    display_light(&dull);
}

```
