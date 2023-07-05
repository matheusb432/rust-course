# Rust 🦀

- Rust is a systems programming language that runs blazingly fast, and enforces memory safety and thread safety to provide a robust foundation for building reliable software.

- Rust's goal is to compile programs into efficient binaries that require as few runtime checks as possible. So checks if a program is safe are made at compile-time

- A foundational goal of Rust is to ensure that your programs `never` have undefined behavior. That is the meaning of "safety." Undefined behavior is especially dangerous for low-level programs with direct access to memory. About 70% of reported security vulnerabilities in low-level systems are caused by memory corruption, which is one form of undefined behavior.

- Rust doesn't have `null` as it uses the `Option<T>` type, which is much more effective in preventing extremely common bugs.

## Why Rust?

- High-level language features without compromising performance
- Program behaviors can be enforced at compile time, leading to enhanced program reliability
- Built-in package manager
- Quickly growing ecosystem
- Friendly community

## Technical Features

- First-class multi-threading
  - Has a compiler error when shared data is improperly accessed
- Powerful type system
- Module system makes code organization easy
- Adding a dependency is as easy as adding a line to a file
- Rust has many tools built-in:
  - _Cargo_: package manager
  - _rustfmt_: code formatter
  - _rustdoc_: documentation generator
  - _rustup_: toolchain manager

## Common Pitfalls

### Sentinel Values

- Sentinel values are values that are used to represent an error state, such as `-1` when a value is not found.
- Since Rust provides the `Result` and `Option` types, sentinel values are an anti-pattern and should never be used.
