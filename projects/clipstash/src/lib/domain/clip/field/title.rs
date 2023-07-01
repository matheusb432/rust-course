use std::str::FromStr;

use super::ClipErr;
use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct Title(Option<String>);

impl Title {
    pub fn new<T: Into<Option<String>>>(value: T) -> Result<Self, ClipErr> {
        let value: Option<String> = value.into();

        let Some(value) = value else {
            return Ok(Self(None));
        };

        match value.trim().len() {
            0 => Ok(Self(None)),
            1..=200 => Ok(Self(Some(value.to_owned()))),
            _ => Err(ClipErr::InvalidTitle("title too large".to_owned())),
        }
    }

    pub fn into_inner(self) -> Option<String> {
        self.0
    }
}

// NOTE This is not needed because of the Default derive macro
// impl Default for Title {
//     fn default() -> Self {
//         Self(None)
//     }
// }

impl FromStr for Title {
    type Err = ClipErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Title {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let res = Self::new(field.value.to_owned());
        Ok(res.map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}
