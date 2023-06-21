# Rust Advanced Concepts

## Iterators

Iterators are lazy and do nothing unless consumed for performance reasons, so using collect() is necessary to run them.

```rust
let numbers = vec![1, 2, 3, 4, 5];

// Calling collect() on an iterator will return a vector
let plus_one: Vec<i32> = numbers.iter().map(|num| num + 1).collect();
```

## Closure

A closure is simply an anonymous function. It can be stored in a variable, passed as an argument to a function, or returned from a function.

```rust

```
