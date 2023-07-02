use super::ClipErr;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize, From, UriDisplayPath, UriDisplayQuery)]
pub struct Shortcode(String);

impl Shortcode {
    pub fn new() -> Self {
        use rand::prelude::*;
        let allowed_chars = ['a', 'b', 'c', 'd', '1', '2', '3', '4'];

        let mut rng = thread_rng();
        // NOTE with_capacity generates a string with a maximum capacity of 10, more efficient than String::new()
        let mut shortcode = String::with_capacity(10);
        for _ in 0..10 {
            shortcode.push(
                *allowed_chars
                    .choose(&mut rng)
                    .expect("sampling array should have values"),
            );
        }
        Self(shortcode)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl Default for Shortcode {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Shortcode> for String {
    fn from(shortcode: Shortcode) -> Self {
        shortcode.into_inner()
    }
}

impl From<&str> for Shortcode {
    fn from(shortcode: &str) -> Self {
        Self(shortcode.to_owned())
    }
}

impl FromStr for Shortcode {
    type Err = ClipErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.to_owned()))
    }
}

use rocket::{request::FromParam, UriDisplayPath, UriDisplayQuery};

impl<'r> FromParam<'r> for Shortcode {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        // ? Equivalent to: Ok(Shortcode::from(param))
        Ok(param.into())
    }
}
