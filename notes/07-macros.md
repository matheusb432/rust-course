# Rust Macros 🦀

- [Macros by example](https://doc.rust-lang.org/reference/macros-by-example.html)

- Macros are a form of metaprogramming, code that writes code.
- Macros in Rust are hygienic:
  - They cannot emit invalid code
  - Data cannot "leak" in to or out of a macro
  - Macros cannot capture information like closures
  - All names / bindings / variables msut be passed in as arguments
- They use macro-specific pattern matching to emit code
- They're invoked with `!` after the macro name
  - Possible to invoke them with square braces or curly braces
  - `macro_name!()` / `macro_name![]` / `macro_name!{}`

## Writing Declarative Macros

- Macros consists of two parts: `Matchers` and `Transcribers`
- `Matchers` are the patterns that match the macro invocation
  - The input patterns are different than patterns used in `match` or `if let`
  - Multiple matchers may be defined for one macro, they're checked in order
- `Transcribers` read the input captured by the matchers and then emit code
  - The transcribed code completely replaces the macro invocation

## Matchers

- `Matchers` consists of four components: `Metavariables`, `Fragment Specifiers`, `Repetitions`, and `Glyphs`.
- All whitespace is ignored in matchers

### Metavariables

- Contain Rust code supplied by the macro invoker
- Used by the transcriber to make substitutions in the output

```rust
$fn
$my_metavar
$varname
```

### Fragment Specifiers

- Fragment specifies determine what kind of data is allowed in a metavariable
- Some of the available specifiers are: `item`, `block`, `stmt`, `pat`, `expr`, `ty`, `ident`, `path`, `tt`, `literal`, `vis`, `lifetime`, `meta`
- Some specifies have restrictions on what can follow to prevent ambiguity
  - Specifiers with restrictions are: `expr`, `stmt`, `ty`, `pat`, `path`, `vis`

#### `item`

- An item is anything that you can make at the top level of a module

```rust
macro_rules! demo {
    // Setting the matcher to accept an item
    ($i:item) => { $i };
}
// Invoking the macro with an item
demo!(const a: char = 'g';);
demo! {fn b() -> char { 'g' }};
demo! {mod demo{}}

struct MyNum(i32);
// implementing MyNum in the macro item
demo! {
    impl MyNum {
        fn new() -> Self {
            Self(0)
        }
    }
}
```

#### `block`

- A block is a set of statements surrounded by curly braces

```rust
macro_rules! demo {
    ($b:block) => { $b };
}

let num = demo!({ if 1 == 1 { 1 + 1 } else { 2 } });
```

#### `stmt`

- `stmt` stands for statement, enables passing in a single statement

```rust
macro_rules! demo {
    ($s:stmt) => { $s };
}

demo!( let a = 1; );
let mut myvec = vec![];
demo!( myvec.push(a); );
```

#### `pat` / `pat_param`

- `pat` is a pattern, used to match against values, it's preferred to use over `pat_param` since it's a newer feature with more functionality

```rust
macro_rules! demo {
    ($p:pat) => {{
        let num = 3;
        match num {
            // Using the pattern in the match expression
            $p => (),
            1 => (),
            _ => (),
        }
    }};
}

demo!( 2 );
```

#### `expr`

- `expr` stands for expression, used to pass in any expression
- a `block` is also an expression

```rust
macro_rules! demo {
    ($e:expr) => { $e };
}
demo!( loop {} );
demo!( 2 + 2 );
demo!( { panic!(); } );
```

#### `ty`

- `ty` stands for type, used to pass in any type

```rust
macro_rules! demo {
    ($t:ty) => {{
        let d: $t = 4;
        fn add(lhs: $t, rhs: $t) -> $t {
            lhs + rhs
        }
    }};
}
demo!(i32);
demo!(usize);
```

#### `ident`

- `ident` stands for identifier, used to pass in any valid identifier
- identifiers will become available after the macro invocation

```rust
macro_rules! demo {
    ($i:ident, $i2:ident) => {
        fn $i() {
            println!("hello");
        }
        let $i2 = 5;
    }
}
demo!(say_hi, five);
// Invoking the identifiers, the macro created a `say_hi` fn and a `five` variable
say_hi();
assert_eq!(five, 5);
```

#### `path`

- `path` is used to pass in any valid path to import a crate or module

```rust
macro_rules! demo {
    ($p:path) => {
        use $p;
    }
}

// Macro will import the std::collections::HashMap module
demo!(std::collections::HashMap);
```

#### `tt`

- `tt` stands for token tree, used to pass in any valid Rust token, can be a single token or a token tree

```rust
macro_rules! demo {
    ($t:tt) => {
        $t {}
    };
}
// Passing the `loop` token
demo!(loop);
// Passing a token tree with curly braces
demo!({
    println!("hello");
})
```

#### `meta`

- `meta` is anything that can be used with a `derive` macro or an attribute macro `#[...]`

```rust
macro_rules! demo {
    ($m:meta) => {
        #[derive($m)]
        struct MyStruct;
    };
}
// Will derive the `Debug` trait for `MyStruct`
demo!(Debug);
```

#### `lifetime`

- `lifetime` is used to pass in any valid lifetime

```rust
macro_rules! demo {
    ($l:lifetime) => {
        let a: &$l str = "hello";
    };
}
demo('static);
```

#### `vis`

- `vis` is used to pass in any valid visibility specifier

```rust
macro_rules! demo {
    ($v:vis) => {
        $v fn sample() {}
    }
}
demo!(pub);
```

#### `literal`

- `literal` is used to pass in any valid literal

```rust
macro_rules! demo {
    ($l:literal) => {
        $l;
    };
}
let five = demo!(5);
let hi = demo!("hello");
```

### Glyphs

- Glyphs are the characters that make up the matcher, they're used to match the input

```rust
macro_rules! demo {
    [W 0 W! _ any | thing? yes.#meta (/./)] => { };
}

// Calling the macro, whitespaces are ignored
demo![W 0 W!_ any | thing?yes.#meta (/./)];
```

## Imports in Macros

- When using external crates in a macro, use the full path prefixed with `::`
  - `use ::std::collections::HashMap`
- When using modules from the current crate, use `$crate`:
  - `$crate::my_module::my_function()`
- This helps resolve import issues since macros can be invoked from any location
