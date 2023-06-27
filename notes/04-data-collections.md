# Rust Data Collections

[When to use each collection](https://doc.rust-lang.org/std/collections/index.html#when-should-you-use-which-collection)

## Iterators

Iterators are lazy and do nothing unless consumed for performance reasons, so using collect() is necessary to run them.

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Calling collect() on an iterator will return a vector
let plus_one: Vec<i32> = numbers.iter().map(|num| num + 1).collect();
```

### iter() and into_iter()

iter() borrows each element of the collection through each iteration, and into_iter() consumes the collection so that each iteration owns the element.

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

Vectors allow you to store lists of data. Vectors can only store values of the same type.

Can add, remove, update elements.

```rust
// The vec! macro expands to something similar to creating the vector and pushing the values into it
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

## String and &str

String is the owned data type, and &str is a string slice.

Must use an owned String to store it in a struct, and it's more efficient to pass &str as a function parameter.

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

## Enums

Enums are not limited to just plain variants, each variant can optionally contain additional data

The additional data is required to create variants on enums that specify a type.

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

A HashMap is a collection that stores key-value pairs, where the key is used to look up the value in an efficient manner.

Similar to a Map in JS.

```rust
let mut people = HashMap::new();
people.insert("John", 30);
people.insert("Becky", 25);
people.insert("Mark", 40);
people.insert("Mark", 50); // Overwrites the previous value
people.remove("Becky");

// Matching on the Result<T> of the get method
match people.get("John") {
    Some(age) => println!("John's age: {}", age),
    None => println!("John not found"),
}

// .iter() return a key-value tuple on HashMaps
for (person, age) in people.iter() {
    println!("{} is {} years old", person, age);
}

// .keys() returns an iterator of the keys
for person in people.keys() {
    println!("Person: {}", person);
}

// .values() returns an iterator of the values
for age in people.values() {
    println!("Age: {}", age);
}
```

## Arrays & Slices

Arrays are a fixed sized memory region that stores values of the same type. Arrays are allocated on the stack. They're useful when working with a fixed buffer size.

```rust
let numbers: [u8; 3] = [1, 2, 3];

// Functions signature types
fn func(arr: [u8; 3]) {}
fn func(arr: &[u8]) {}
fn func(arr: &mut [u8]) {}
```

A slice is a borrowed view into an array, they can be iterated upon and are optionally mutable.

Slices can be obtained from any data structure that's backed by an array, such as vectors.

```rust
let numbers: [u8; 3] = [1, 2, 3, 4, 5];

let slice: &[u8] = &numbers;

// Slicing a specific range of values
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
    // ! This match arm will never match, slice patterns easily overlap
    [.., second] => (),
    [] => (),
}

// This works, matching on larger patterns first
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

Borrowing a Vector as an argument to a function that requires a slice will automatically obtain a slice, in general it should be preferred to borrow a slice instead of a vector.

```rust
fn func(slice: &[u8]) {}

let numbers = vec![1, 2, 3];
// Will automatically borrow a slice from the vector
func(&numbers);
let numbers: &[u8] = numbers.as_slice();
```
