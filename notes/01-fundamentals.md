# Rust Fundamentals 🦀

Rust is an expression-based language, this means that most things are evaluated and return some value, anything that doesn't return a value is a statement.

```rust
// `let x` is a statement, `5 + 5` is an expression
let x = 5 + 5;
```

In general, writing Rust functions is a careful balance of asking for the right level of permission.

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

### Constants

Constants are similar to variables, but they are immutable and must be annotated with a type, they must also have a value that can be computed at compile time.

```rust
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
```

Constants are more performant than variables because they are inlined wherever they are used.

### Shadowing

Shadowing lets us transform a variable in a way that is clear, type-safe, and performant. It also helps keep our code concise and self-documenting, reducing the mental overhead of tracking multiple variable names.

```rust
let spaces = " "; // spaces is a string
let spaces = spaces.len(); // now spaces is an integer, the length of the original string
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

## Expressions

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

## Closure

A closure is simply an anonymous function. It can be stored in a variable, passed as an argument to a function, or returned from a function.

```rust

// A closure that takes no arguments and returns nothing
let say_hello = || println!("Hello!");

// A closure that takes two arguments and returns the sum of them
let sum = |a, b| -> i32 { a + b };
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

You can return values out of a loop by adding an expression after the `break` keyword, like so:

```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        // Will return 20 and break out of the loop
        break counter * 2;
    }
};
println!("The result is {result}"); // "The result is 20"
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

## Modules

Modules are a way to organize code and control the privacy of paths. A module is a collection of items: functions, structs, traits, impl blocks, and even other modules.

```rust
mod my_module {
    // Items in modules default to private visibility
    fn private_function() {
        println!("called `my_module::private_function()`");
    }

    // Use the `pub` modifier to override default visibility.
    pub fn function() {
        println!("called `my_module::function()`");
    }
}
```

### Crates

A crate can come in one of two forms: a binary crate or a library crate. A binary crate is an executable, and a library crate is a collection of code that can be imported into other crates.

## Overloading traits

### Equality & Ordering

- When comparing structs with the PartialEq, all fields must be exactly the same to return true on a `a == b` comparison, PartialOrd trait, only the first field of a struct is compared in ordering comparisons
- This enables easier sorting and comparison of structs

### Operators

- Operators can be overloaded by implementing operator traits

```rust
use std::ops::Add;

struct Speed(u32);

impl Add<Self> for Speed {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Speed(slef.0 + rhs.0)
    }
}

// This is now possible and will call the add method of the implementation
let fast = Speed(5) + Speed(3); // Speed(8)

// Implementations for different types can be done since the operator trait is generic
impl Add<u32> for Speed {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        Speed(self.0 + rhs)
    }
}
let fast = Speed(5) + 3; // Speed(8)
```

Indexing can also be overloaded by implementing the `Index` trait

```rust
use std::ops::Index;

struct Hvac {
    current_temp: i16,
    min_temp: i16,
    max_temp: i16,
}
enum Temp {
    Current,
    Max,
    Min,
}
impl Index<Temp> for Hvac {
    type Output = i16;

    fn index(&self, temp: Temp) -> &Self::Output {
        match temp {
            Temp::Current => &self.current_temp,
            Temp::Max => &self.max_temp,
            Temp::Min => &self.min_temp,
        }
    }
}
let env = Hvac {
    current_temp: 20,
    min_temp: 15,
    max_temp: 25,
};
// Now we can index the struct with the implemented enum
let current = env[Temp::Current]; // 20
```

### Iterators Traits

- Iteration is provided by the `Iterator` trait, it's what provides the `for..in` syntax and access to the iterator adapters `map, filter, ...`
- Can be implemented for any struct

```rust
// Iterator trait signature
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

- The `IntoIterator` trait is used to convert a type into an iterator, it's implemented for types that can be used as collections, like arrays, vectors, and strings

```rust
// IntoIterator trait signature
trait IntoIterator
{
    type Item;
    type IntoIter;

    fn into_iter(self) -> Self::IntoIter;
}
```
