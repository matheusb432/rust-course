// Topic: Macro practice
//
// Summary:
//   Create a macro that can be used to generate new test cases for
//   the function provided.
//
// Requirements:
// * 1. Write a macro to generate tests for `sample_fn`
// * 2. Create at least 6 test cases using the macro
//    * Test the minimum and maximum values for each match arm
// * 3. All test functions must be created by invoking the macro
//
// Notes:
// * Tuples can be used to specify both the input and expected output
// * The macro can be invoked multiple times; repetitions are optional

#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Size {
    Small,
    Medium,
    Large,
}

fn sample_fn(n: u8) -> Size {
    use Size::*;
    match n {
        0..=53 => Small,
        54..=154 => Medium,
        155.. => Large,
    }
}

fn main() {
    // use `cargo test --bin m5` to check your work
}

#[cfg(test)]
mod test {
    use super::*;
    use Size::*;

    // ? 1.
    macro_rules! multi_test {
        // * fn_to_test: test_name -> (input, expected)
        ($fn:ident : $( $name:ident -> $values:expr ),+ $(,)?) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($fn($values.0), $values.1)
                }
            )+
        };
        // ? test_name -> (assertion statement)
        ($( $name:ident -> $assert_stmt:stmt ),+ $(,)?) => {
            $(
                #[test]
                fn $name() {
                    $assert_stmt
                }
            )+
        };
    }

    macro_rules! test_fn {
        ($fn: ident -> $(($input: expr, $expected: expr)),+ $(,)?) => {
            $(
                assert_eq!($fn($input), $expected);
            )+
        };
    }

    // ? 2. 3.
    multi_test!(sample_fn:
         test_small -> (10, Small),
         test_medium -> (60, Medium),
         test_large -> (155, Large),
    );

    // ? Creates a test_all test function that will run the `test_fn!(..) statement
    multi_test!(test_all ->
        test_fn!(sample_fn -> (10, Small), (60, Medium), (210, Large),)
    );
}
