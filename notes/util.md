# Common CLI Commands

## Cargo

`cargo init`

> Creates a new project.

`cargo run`

> Compiles and runs the main binary file.

`cargo run -q --bin {binary file name}`

> Runs a binary file without printing the `cargo run` output.

`cargo add {crate name}{@version?}`

> Adds a crate to the project.

## Clippy

`cargo clippy --bin {binary file name}`

> Lints a binary file.

`cargo clippy --all-targets`

> Lints the entire project.

`cargo clippy --fix --workspace --all-targets --allow-staged`

> Fixes lints as long as there are no unstaged changes.
