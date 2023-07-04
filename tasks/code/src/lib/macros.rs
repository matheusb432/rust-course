// NOTE Exporting a macro in a module
#[macro_export]
macro_rules! hmap {
    ($($key: expr => $value: expr),* $(,)?) => {{
        let mut hashmap = ::std::collections::HashMap::new();

        $(
            hashmap.insert($key, $value);
        )*
        hashmap
    }}
}
