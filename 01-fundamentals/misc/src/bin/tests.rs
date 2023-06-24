fn all_caps(s: &str) -> String {
    s.to_uppercase()
}

fn main() {}

#[cfg(test)]
mod tests {
    // NOTE `use crate::*` brings all items in the crate into scope
    use crate::*;

    // NOTE defining a test function
    #[test]
    fn check_all_caps() {
        let res = all_caps("hello");
        let expected = String::from("HELLO");
        // NOTE the program is aborted if assert fails, which will result in a failed test
        assert_eq!(res, expected, "string should be all caps");
    }
}
