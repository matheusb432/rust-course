# Rust Fundamentals

Rust is an expression-based language, this means that most things are evaluated and return some value

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

## Control Flow

### If statements

```rust
let x = 5;

// No parentheses around the condition
if x == 5 {
    println!("x is five!");
} else if x == 6 {
    println!("x is six!");
} else {
    println!("x is not five or six :(");
}
```

### Loops

```rust
let mut x = 0;

// loop is an infinite loop until a break statement is reached
loop {
    if x == 5 {
        break;
    }

    // Prints 0 through 4
    println!("{x:?}");
    x += 1;
}

// Executes while the condition is true
while x > 0 {
    // Prints 5 through 1
    println!("{x:?}");
    x -= 1;
}
```

### Match expressions

Match expressions are a much more powerful switch statemen

Match expressions must be exhaustive, they should account for every possible result, so if a match is used for controlling an action on an enum value, the compiler throw an error if that is not accounted for

```rust
let some_bool = true;

match some_bool {
    true => println!("Is true"),
    false => println!("Is false"),
}

let some_int = 5;
match some_int {
    1 => println!("Is one"),
    2 => println!("Is two"),
    3 => println!("Is three"),
    // NOTE _ is the default case
    _ => println!("Is something else"),
}
```

## Data Types

Type annotations are mostly optional within function bodies, they can be explicitly specified using let bindings, but it's usually better to let the compiler infer the types

### Struct

A struct (structure) is a type that contain multiple fields to group similar data together, much like an object

It cannot have some pieces of data and not others.

Structs cannot have borrowed `&` data, as they are responsible for cleaning up their data when they are dropped, and they cannot drop data that they don't own.

```rust
struct ShippingBox {
    depth: i32,
    width: i32,
    height: i32,
}

let my_box = ShippingBox {
    depth: 3,
    width: 2,
    height: 5,
};

println!(my_box.depth) // 3
```

### Tuples

A type of record to store data anonymously (unnamed fields) that can be easily destructured into variables

Ideally tuples should have at most 3 values, if more values than that are needed, structs should be used instead

Useful to return pairs of data from functions

```rust
enum Access {
    Full,
}

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}

let numbers = one_two_three();
println!("numbers -> {:?}", numbers);
let (employee, access) = ("John", Access::Full);
```

### Expressions

Expression values coalesce to a single point, they can be used for nesting logic

It's also possible to nest expressions, but it should be limited to 2 or 3 levels of nesting so the code doesn't become too complex

```rust
let my_num = 3;
// Setting is_lt_5 to the result of the expression
let is_lt_5 = if my_num < 5 {
    true
} else {
    false
};

// Equivalent since the ternary expression would be redundant
let is_lt_5 = my_num < 5;

// Possible to set message to the result of the match expression
let message = match my_num {
    1 => "hello",
    _ => "goodbye",
}
```

### Option

Option is a type that can either be some data of a specified type, or nothing

Is used in scenarios where data may not be required or is unavailable

```rust
// The option type in the default Rust library
enum Option<T> {
    Some(T),
    None
}
```

Examples of structs with Option fields

```rust
struct Customer {
    age: Option<i32>,
    email: String,
}

let mark = Customer {
    // Providing the age with Some
    age: Some(30),
    email: "example@email.com".to_owned(),
}
let becky = Customer {
    // Even though age is optional, it's still required to be explicitly None
    age: None,
    email: "example@email.com".to_owned(),
}

match becky.age {
    // Matching on an Option type, the value is extracted from the Some variant
    Some(age) => println!("Customer age: {age}"),
    None => println!("Customer age not provided"),
}
```

Functions that return optional data:

```rust
struct GroceryItem {
    name: String,
    qty: i32,
}

fn find_quantity(name: &str) -> Option<i32> {
    let grocery_list = vec![
        GroceryItem {
            name: "bananas".to_owned(),
            qty: 5,
        },
        GroceryItem {
            name: "apples".to_owned(),
            qty: 3,
        },
    ];

    for item in grocery_list {
        if item.name == name {
            // Having an early return with the found value
            return Some(item.qty);
        }
    }
    // Explicitly returning None if the value is not found
    None
}
```
