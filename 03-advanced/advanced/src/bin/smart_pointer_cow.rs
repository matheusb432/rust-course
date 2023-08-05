use std::borrow::Cow;

// NOTE Cow (clone-on-write) is a smart pointer that allows for cheaply cloning data when it's immutable, and lazily cloning it when it's mutable
fn make_separator(user_str: &str) -> Cow<'_, str> {
    if !user_str.is_empty() {
        user_str.into()
    } else {
        Cow::Borrowed("==========")
    }
}

fn main() {
    let cow = make_separator("");
    dbg!(cow);
    let cow = make_separator("greetings");
    dbg!(cow);
}
