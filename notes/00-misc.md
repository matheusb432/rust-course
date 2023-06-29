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

### derive_more

- The `derive_more` crate is a crate that allows you to derive more traits for structs and enums.
- It provides the `From` trait for enums, the `Add` trait for structs, and more.

### rayon

- The `rayon` crate is a crate that allows you to easily parallelize code, with automatic multithreading.
- No boilerplate needed to enable parallel processing.
- Automatically users all processing cores available on the machine

### tracing

- The `tracing` crate is a crate that allows you to instrument your code with tracing events.
- It is useful for logging, debugging and performance analysis.

### color-eyre

- The `color-eyre` crate is a crate that allows you to easily standardize error handling in your code.
- It is useful for debugging and logging.
