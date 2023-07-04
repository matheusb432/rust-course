// Topic: Basic macro repetitions
//
// Requirements:
//   * 1. Create a macro to generate hashmaps.
//   * 2. The macro must be able to accept multiple key/value pairs.
//   * 3. Print out the generated hashmap using the `dbg!` macro to ensure it works.

// ? 1. 2.
// NOTE Hash map macro
macro_rules! hash_map {
    (
        $($key: expr => $value:expr),*
        $(,)?
    ) => {{
      use ::std::collections::HashMap;
      let mut hm = HashMap::new();
      $(
          hm.insert($key, $value);
      )*
      hm
    }};
}

// ? As JS object syntax, but `tt` is less versatile than `expr`
macro_rules! hmap {
    (
        $($key: tt: $value: expr),*
        $(,)?
    ) => {{
        use ::std::collections::HashMap;
        let mut hm = HashMap::new();
        $(
            hm.insert($key, $value);
        )*
        hm
    }};
}

fn main() {
    let new_map = hash_map! {
       "apple" => 10, "oranj" => 15, "banan" => 300
    };
    let alt_map = hmap! { "appl": 30, "hello": 50, };
    // ? 3.
    dbg!(&new_map.get("apple"));
    dbg!(new_map);
    dbg!(alt_map);
}
