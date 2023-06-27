# Rust Data Types 🦀

Type annotations are mostly optional within function bodies, they can be explicitly specified using let bindings, but it's usually better to let the compiler infer the types

## Scalar Types

A scalar type represents a single value. Rust has four primary scalar types: ints, floats, bools, and chars.

### Integers

the isize and usize types depend on the architecture of the computer your program is running on, as in “arch” types: 64 bits on a 64-bit architecture and 32 bits on a 32-bit architecture.

### Floats

The default float type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

## Type Aliases

Type aliases are a way to give a new name to an existing type. They are declared using the type keyword, they are most useful to alias long type names to more succinct ones.

```rust
struct Contact {
    name: String,
    phone: String,
}

type ContactName = String;
type ContactIndex = HashMap<ContactName, Contact>;

fn add_contact(index: &mut ContactIndex, contact: Contact) {
    index.insert(contact.phone.to_owned(), contact);
}

// Aliasing a generic type
type GenericThings<T> = Vec<Thing<T>>;
```

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

## Impl

Implementations allows you to define methods that operate on instances of a struct or an enum, as well as associated functions that do not operate on an instance but are associated with the type. Additionally, it's used for implementing traits for specific types.

Implementations enable the usage of OOP patterns.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

let rect1 = Rectangle { width: 30, height: 50 };

println!("The area of the rectangle is {} square pixels.", rect1.area());
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

## Traits

Traits are a way to define shared behavior between types. Traits are similar to interfaces in other languages.

Traits can only access data through functions that are defined in the trait, they are unable to access fields directly.

```rust
trait Noise {
    fn make_noise(&self);
}

struct Person;
// Implementing the trait Noise for the struct Person
impl Noise for Person {
    // The trait method signature must be the same as the one defined in the trait
    fn make_noise(&self) {
        println!("hello");
    }
}

struct Dog;
impl Noise for Dog {
    fn make_noise(&self) {
        println!("woof");
    }
}

fn hello(noisy: impl Noise) {
    noisy.make_noise();
}

fn main() {
    hello(Person {});
    hello(Dog {});
}
```

## Generics

Generic types are a way to define a type that can be used with multiple other types.

```rust
fn function<T: Trait1, U: Trait2>(param1: T, param2: U) {
    // ...
}

// `where` syntax, usually better to use when there's two or more generic types
fn function<T, U>(param1: T, param2: U)
where
    // NOTE T must implement both Trait1 and Trait2
    T: Trait1 + Trait2,
    U: Trait1 + Trait2 + Trait3,
{
    // ...
}
```

Refactoring a generic function so it's type can be used elsewhere:

```rust
fn hello(noisy: impl Noise) {
    noisy.make_noise();
}
// Refactored to a generic function
fn hello<T: Noise>(noisy: T) {
    noisy.make_noise();
}
```

Rust uses monomorphization to generate code for each concrete type used with a generic type, meaning that the generic type is replaced with the concrete type at compile time.

This leads to an increase in the binary size, but also an increase in performance since the compiler can optimize the code for each concrete type.

```rust

// Generic function
fn hello<T>(noisy: T) {
    noisy.make_noise();
}

// Concrete use
hello(Person {});

// The compiler will generate the following code at compile time
fn hello(noisy: Person) {
    noisy.make_noise();
}
```

Generic structures:

```rust
struct Point<T> {
    x: T,
    y: T,
}

trait Seat {
    fn show(&self);
}
struct Ticket<T: Seat> {
    seat: T,
}
```

## Trait Objects

- Trait objects offer a way to dynamically change program behavior at runtime, at the cost of performance.
- They use "dynamic dispatch" to determine which method to call at runtime. In contrast to Generics which use static dispatch.
- They allow for mixed types in a collection and polymorphism.
  - It's ideal to only use them for small collections, since they can be much slower than generics.

```rust
trait Clicky {
    fn click(&self);
}

struct Keyboard;

impl Clicky for Keyboard {
    fn click(&self) {
        println!("clicked");
    }
}

let kb = Keyboard;
let kb_obj: &dyn Clicky = &kb;

// Alternate way to create a trait object
let kb: &dyn Clicky = &Keyboard;

// Creating a trait object from a Box for more flexibility
let kb: Box<dyn Clicky> = Box::new(Keyboard);

// Setting a function that accepts a trait object that implements the Clicky trait
fn borrow_clicky(obj: &dyn Clicky) {
    obj.click();
}

let kb = Keyboard;
borrow_clicky(&kb);

fn move_clicky(obj: Box<dyn Clicky>) {
    obj.click();
}

let kb = Box::new(Keyboard);
move_clicky(kb);
```

### Creating a heterogeneous collection

```rust
struct Mouse;
impl Clicky for Mouse {
    fn click(&self) {
        println!("clicked");
    }
}

let kb: Box<dyn Clicky> = Box::new(Keyboard);
let mouse: Box<dyn Clicky> = Box::new(Mouse);
let clickers = vec![kb, mouse];

// Alternate way to create a heterogeneous collection, more concise
let kb = Box::new(Keyboard);
let mouse = Box::new(Mouse);
let clickers: Vec<Box<dyn Clicky>> = vec![kb, mouse];
```
