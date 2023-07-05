# Rust Data Collections 🦀

[When to use each collection](https://doc.rust-lang.org/std/collections/index.html#when-should-you-use-which-collection)

## Iterators

- Iterators are one of Rust's _zero-cost abstractions_, they can sometimes even be faster than imperative code.
- Iterators are 'lazy' for performance reasons, so using collect() is necessary to run them.

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Calling collect() on an iterator will return a vector
let plus_one: Vec<i32> = numbers.iter().map(|num| num + 1).collect();
```

### `iter()` and `into_iter()`

- `iter()` borrows each element of the collection through each iteration, and `into_iter()` consumes the collection so that each iteration owns the element.

```rust
let numbers = vec![1, 2, 3, 4, 5];

// borrows each element of the collection through each iteration
for num in numbers.iter() {
    dbg!(num); // num -> &i32
}

// consumes the collection so that each iteration owns the element
for num in numbers.into_iter() {
    dbg!(num); // num -> i32
}
```

## Vector

- Vectors allow you to store dynamic, mutable lists of data.
- Vectors can only store values of the same type.

```rust
// The vec! macro expands to creating the vector and pushing the values into it
let my_numbers = vec![1, 2, 3];

let mut my_numbers = Vec::new();
my_numbers.push(1);
my_numbers.push(2);
my_numbers.push(3);
my_numbers.pop();
my_numbers.len(); // 2

// Acessing a vector value using the slice notation, by it's index
let two = my_numbers[1];

// Iterating in vector values
for num in my_numbers {
    println!("{:?}", num);
}
```

## `String` and `&str`

- `String` is the owned data type, and `&str` is a string slice.
- It's more efficient to pass `&str` as a function parameter so that no data is copied.

```rust
fn print_it(input: &str) {
    println!("{}", input);
}

fn main() {
    // The string in quotes is automatically borrowed as a &str
    print_it("a string slice");

    let owned_string = "owned string".to_owned();
    let another_owned = String::from("another owned string");

    // Passing String types as their references
    print_it(&owned_string);
    print_it(&another_owned);
}
```

## Tuples

- A tuple is a record to store data anonymously (unnamed fields) that can be easily destructured into variables
- Ideally tuples should have at most 3 values, structs should be used for more values

```rust
enum Access {
    Full,
}
let (employee, access) = ("John", Access::Full);

fn one_two_three() -> (i32, i32, i32) {
    (1, 2, 3)
}
let numbers = one_two_three();
```

## Enums

- Enums are a type that can have a fixed set of variants, each variant can optionally contain additional data.

```rust
enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    // Scroll can represent the mouse scroll amount
    Scroll(i32),
    // Move can represent the mouse movement in x and y
    Move(i32, i32),
}

enum PromoDiscount {
    NewUser,
    Holiday(String),
}
enum Discount {
    Percent(i32),
    Flat(i32),
    // It's possible to have enums within enums
    Promo(PromoDiscount),
    Custom(String),
}
```

## HashMap

- A HashMap is a collection that stores key-value pairs (KVPs), where the key is used to look up the value in an efficient manner. It's similar to a Map in JS.

```rust
let mut people = HashMap::new();
people.insert("John", 30);
people.insert("Becky", 25);
people.insert("Mark", 40);
people.insert("Mark", 50); // Overwrites the previous value
people.remove("Becky");

// The get() method returns an Option<T>
match people.get("John") {
    Some(age) => println!("John's age: {}", age),
    None => println!("John not found"),
}

// .iter() return a KVP tuple on HashMaps, keys() and values() return iterators
for (person, age) in people.iter() {
    println!("{} is {} years old", person, age);
}
for person in people.keys() {
    println!("Person: {}", person);
}
for age in people.values() {
    println!("Age: {}", age);
}
```

## Arrays & Slices

- Arrays are a fixed sized memory region that stores values of the same type. Arrays are allocated on the stack. They're useful when working with a fixed buffer size.

- A slice is a borrowed view into an array, they can be iterated upon and are optionally mutable.
- Slices can be obtained from any data structure that's backed by an array, such as vectors.

```rust
let numbers: [u8; 5] = [1, 2, 3, 4, 5];

// Functions signature types
fn func(arr: [u8; 5]) {}
fn func(arr: &[u8]) {}
fn func(arr: &mut [u8]) {}

let slice: &[u8] = &numbers;

// Slicing a specific range of values by index
let from_one_through_two: &[u8] = &numbers[1..=2];
let from_zero_through_two: &[u8] = &numbers[..3];

let chars = vec!['a', 'b', 'c', 'd', 'e'];
match chars.as_slice() {
    // Possible to get first and last elements like so
    // If the slice only has 2 elements, `..` will not have any values
    [first, .., last] => (),
    [first, second, ..] => (),
    [single] => (),
    [] => (),
}

match chars.as_slice() {
    [first, ..] => (),
    // ! This match arm will NEVER match, slice patterns easily overlap
    [.., second] => (),
    [] => (),
}

// This works, match on larger patterns first
match chars.as_slice() {
    // Will only match if the slice as 3 or more elements, then 2 or more and so on
    [first, second, third, ..] => (),
    [first, second, ..] => (),
    [first, ..] => (),
    [] => (),
}

// Using guards
match chars.as_slice() {
    // Will match if first is 1, 2 or 3, 'rest' is a slice of the remaining elements
    [first @ 1..=3, rest @ ..] => (),
    [single] if single == &5 || single == &6 => (),
    [first, second] => (),
    // Exhaustive patterns so all cases are covered
    [..] => (),
    [] => (),
}
```

- Borrowing a Vector as an argument to a function that requires a slice will automatically obtain a a slice of the entire collection, in general it should be preferred to borrow a slice instead of a vector.

```rust
fn func(slice: &[u8]) {}

let numbers = vec![1, 2, 3];
// Will automatically borrow a slice from the vector
func(&numbers);
let numbers: &[u8] = numbers.as_slice();
```
