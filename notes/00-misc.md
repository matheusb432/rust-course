# Misc

## Useful Cargo Crates

### Clippy

- Clippy is a linter for Rust
- Running `cargo clippy --bin {binary file name}` will lint code in a binary file

### dotenvy

- The `dotenvy` crate is a crate that allows you to load environment variables from a `.env` file.

### rand

- The `rand` crate is a crate that allows you to generate random numbers.
- It also provides cryptographically secure RNGs via the `thread_rng` function.

### cached

- The `cached` crate is a crate that allows you to cache the results of a function.
- It is most useful when useful with computationally expensive functions.
- Provides TTL (time to live) and LRU (least recently used) caching strategies.
- Since the cache is stored in a `HashMap`, the cached function must return owned values.

### strum

- The `strum` crate is a crate that allows you to create enums that can be converted to and from strings.
- It also provides the `EnumIter` trait which allows you to iterate over the variants of an enum.
- Mostly useful for parsing command line arguments.

```rust
use strum_macros::{EnumIter, EnumString, EnumVariantNames};

#[derive(Debug, EnumIter, EnumString, EnumVariantNames)]
enum MyEnum {
    Foo,
    Bar,
    Baz,
}
let my_enum = MyEnum::from_str("Foo").unwrap();
println!("{:?}", my_enum); // Foo
println!("{:?}", MyEnum::VARIANTS); // ["Foo", "Bar", "Baz"]

// Serialize string to enum
#[derive(Debug, EnumString)]
enum Status {
    #[strum(serialize = "i", serialize = "Idle")]
    Idle,
    #[strum(serialize = "p")]
    Processing,
}
let idle = Status::from_str("i"); // Ok(Status::Idle)
let processing = Status::from_str("p"); // Ok(Status::Processing)
let processing = Status::from_str("Processing"); // Err(NotFound)
```
