// Topic: TryFrom/TryInto
//
// Summary:
// * A library is needed for an application to convert hex color codes
//   into their component color values (red, green, and blue). Hex color codes
//   consist of a hash symbol followed by six hex digits. Every two hex digits
//   represent a color component in the order of red, green, blue.
//
//   Example hex color codes:
//    #ffffff -> Rgb(255, 255, 255)
//    #001122 -> Rgb(0, 17, 34)
//
// Requirements:
// * 1. Create a program to convert a hex code (as &str) into an Rgb struct
// * 2. Implement TryFrom to perform the conversion
// * 3. Utilize the question mark operator in your implementation
//
// Notes:
// * See the `from_str_radix` function in the stdlib docs for `u8`
//   to convert hex digits to `u8`
//   * Hex digits use a radix value of 16
// * Utilize the `thiserror` crate for your error type
// * Run `cargo test --bin a37` to test your implementation

use std::num::ParseIntError;

use thiserror::Error;

#[derive(Error, Debug)]
enum RgbError {
    #[error("Could not parse {0} to a hexcode")]
    ParseInt(#[from] ParseIntError),
    #[error("Invalid hexcode: {0}")]
    InvalidHexcode(&'static str),
}

#[derive(Debug, Eq, PartialEq)]
struct Rgb(u8, u8, u8);

const INVALID_LENGTH: &'static str = "Invalid length";
const MISSING_HASH: &'static str = "Missing hash";

impl From<&'static str> for RgbError {
    fn from(value: &'static str) -> Self {
        RgbError::InvalidHexcode(value)
    }
}

impl TryFrom<&str> for Rgb {
    type Error = RgbError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if &value[0..1] != "#" {
            return Err(RgbError::from(MISSING_HASH));
        }

        let (first, second, third) = match &value[1..] {
            s if s.len() != 6 => {
                return Err(RgbError::from(INVALID_LENGTH));
            }
            s => (&s[..2], &s[2..4], &s[4..6]),
        };

        Ok(Rgb(to_hex(first)?, to_hex(second)?, to_hex(third)?))
    }
}

fn to_hex(value: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(value, 16)
}

fn main() {
    let rgb: Result<Rgb, RgbError> = "#0024ff".try_into();
    println!("{:?}", rgb)
}

#[cfg(test)]
mod test {
    use super::Rgb;
    use std::convert::TryFrom;

    #[test]
    fn converts_valid_hex_color() {
        let expected = Rgb(0, 204, 102);
        let actual = Rgb::try_from("#00cc66");
        assert_eq!(
            actual.is_ok(),
            true,
            "valid hex code should be converted to Rgb"
        );
        assert_eq!(actual.unwrap(), expected, "wrong Rgb value");
    }

    #[test]
    fn fails_on_invalid_hex_digits() {
        assert_eq!(
            Rgb::try_from("#0011yy").is_err(),
            true,
            "should be an error with invalid hex color"
        );
    }

    #[test]
    fn fails_when_missing_hash() {
        assert_eq!(
            Rgb::try_from("001100").is_err(),
            true,
            "should be an error when missing hash symbol"
        );
    }

    #[test]
    fn fails_when_missing_color_components() {
        assert_eq!(
            Rgb::try_from("#0011f").is_err(),
            true,
            "should be an error when missing one or more color components"
        );
    }

    #[test]
    fn fails_with_too_many_color_components() {
        assert_eq!(
            Rgb::try_from("#0011ffa").is_err(),
            true,
            "should be an error when too many color components are provided"
        );
    }
}
