# Rust Advanced Concepts

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

## Closure

A closure is simply an anonymous function. It can be stored in a variable, passed as an argument to a function, or returned from a function.

```rust

```
