fn maybe_enum() -> Option<i32> {
    Some(1)
}

fn maybe_word() -> Option<String> {
    Some("hello".to_string())
}

fn main() {
    // let plus_one = match maybe_enum() {
    //     Some(x) => x + 1,
    //     None => None,
    // };

    //  NOTE Using the map combinator to have more concise code, map() only runs when maybe_enum() return Some<T> value
    let plus_one = maybe_enum().map(|x| x + 1);
    let word_length = maybe_word().map(|word| word.len()).map(|len| len * 2);

    println!("plus_one: {:?}\nword_length: {:?}", plus_one, word_length);
}
