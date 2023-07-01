use super::ClipErr;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Default, Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub struct Password(Option<String>);

impl Password {
    // NOTE This generic type can accept either an Option<String> or a String
    pub fn new<T: Into<Option<String>>>(value: T) -> Result<Self, ClipErr> {
        // ? Will wrap the value into an Option if it's a String
        let value: Option<String> = value.into();

        match value {
            Some(value) if !value.trim().is_empty() => Ok(Self(Some(value))),
            _ => Ok(Self(None)),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }

    pub fn has_password(&self) -> bool {
        self.0.is_some()
    }

    pub fn is_valid(&self, other: &Self) -> bool {
        !self.has_password() || self == other
    }
}

impl FromStr for Password {
    type Err = ClipErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_owned())
    }
}
