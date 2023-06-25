# Rust Data Types

Type annotations are mostly optional within function bodies, they can be explicitly specified using let bindings, but it's usually better to let the compiler infer the types

## Scalar Types

A scalar type represents a single value. Rust has four primary scalar types: ints, floats, bools, and chars.

### Integers

the isize and usize types depend on the architecture of the computer your program is running on, as in “arch” types: 64 bits on a 64-bit architecture and 32 bits on a 32-bit architecture.

### Floats

The default float type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

## Struct

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

## Tuples

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

## Option

Option is a type that can either be some data of a specified type, or nothing

Is used in scenarios where data may not be required or is unavailable

```rust
// The option type in the standard Rust library
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

## Result

Result is a type that can either be some data of a specified type (Ok), or an error (Err)

Is used when an action needs to be taken, but has the possibility of failure (e.g. reading a file, HTTP request, etc.)

```rust
// The result type in the standard Rust library
enum Result<T, E> {
    // Ok with the success value of type T
    Ok(T),
    // Err with the error of type E
    Err(E),
}
```

Usage of the Result type:

```rust
fn get_sound(name: &str) -> Result<SoundData, String> {
    if name == "alert" {
        // Returning the success value with Ok
        Ok(SoundData::new("alert")),
    } else {
        // Returning the error value with Err
        Err("Sound not found".to_owned())
    }
}

let sound = get_sound("alert");

// Matching on the result type to handle both the error and success cases concisely
match sound {
    Ok(_) => println!("Sound data located"),
    Err(err) => println!("Error: {:?}", err),
}
```
