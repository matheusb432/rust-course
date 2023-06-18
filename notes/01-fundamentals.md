# Rust Fundamentals

## Data Types

- Memory only stores binary data
- Code is compiled to binary representation
- Basic types are provided by the language
  - Boolean, Integer, Double/Float, Character, String

## Variables

Variables are a way to assign data to a temporary memory location.

```rust
let price = 30;
let city = "NYC";
```

By default, variables are immutable. To make a variable mutable, use the `mut` keyword.

```rust
let mut discount_price = 49.99;
let mut name = "John";
```

## Functions

In Rust, returns are implicit. The last expression in a function is the return value.

The type of the return value is defined with `->`

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}

let res = add(5, 10); // 15
```

## Macros

In Rust, a macro is a way of defining reusable chunks of code. Macros in Rust are a bit similar to functions, but instead of generating a value, macros generate code snippets.

```rust
let life = 42;
println!("hello"); // hello
// The `:?` is for debugging purposes
println!("{:?}", life); // 42
println!("{life:?}"); // 42
// Without the `:?` it can be used for end users
println!("For end user: {life}"); // 42
println!("The meaning of {:?} is {:?}", life, life); // The meaning of 42 is 42
```

Creating a macro

```rust
macro_rules! say_hello {
    () => (
        // println! is also a macro
        println!("Hi!");
    )
}

...

say_hello!() // Output: "Hi!"
```

One of the main advantages of macros over functions in Rust is that they are more flexible and can accept a varying number of arguments, among other things.

Macros work through a process called macro expansion, where the macro is expanded to its body code before the code is compiled.

Macros can also be more difficult to write and debug than functions, so they should be used judiciously.
