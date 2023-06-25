# Rust 🦀

Rust is a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety.

Rust's goal is to compile programs into efficient binaries that require as few runtime checks as possible. So checks if a program is safe are made at compile-time

A foundational goal of Rust is to ensure that your programs never have undefined behavior. That is the meaning of "safety." Undefined behavior is especially dangerous for low-level programs with direct access to memory. About 70% of reported security vulnerabilities in low-level systems are caused by memory corruption, which is one form of undefined behavior.

## Why Rust?

- High-level language features without compromising performance
- Program behaviors can be enforced at compile time
  - Enhanced program reliability
- Built-in package manager
- Quickly growing ecosystem
- Friendly community

## Technical Features

- First-class multi-threading
  - Has a compiler error when shared data is improperly accessed
- Powerful type system
- Module system makes code organization easy
- Adding a dependency is as easy as adding a line to a file
- Tooling is built-in
  - Cargo: package manager
  - rustfmt: code formatter
  - rustdoc: documentation generator
  - rustup: toolchain manager
