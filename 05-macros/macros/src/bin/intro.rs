#![allow(dead_code, unused_variables)]

// NOTE Creating a macro to set a constant
macro_rules! constant {
    // NOTE ($name: ident) is a Matcher
    ($name:ident) => {
        // NOTE the code here is the Transcriber
        const $name: &'static str = "macro const!";
    };
}

macro_rules! newtype {
    ($name:ident, $typ: ty) => {
        struct $name($typ);
    };
}

// NOTE Associated item macros
macro_rules! msg {
    ($msg:literal) => {
        pub fn msg() {
            println!("macro msg: {}", $msg);
        }
    };
}

// NOTE macro_rules Transcribers - Composing macros
macro_rules! print_demo {
    () => {
        println!("{}", format!("demo{}", '!'))
    };
}

struct Demo;
impl Demo {
    // ? Will implement a method called `msg`
    msg!("Hello, world!");
}

fn main() {
    // Expression macros
    let nums = vec![1, 2, 3];

    // Statement macros
    println!("Hello, world!");

    constant!(HELLO_WORLD);
    assert_eq!(HELLO_WORLD, "macro const!");
    println!("{}", HELLO_WORLD);

    newtype!(DemoStruct, usize);
    let demo = DemoStruct(42);
    Demo::msg();
    print_demo!();
}
