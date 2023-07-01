use super::ClipErr;
use rocket::form::{self, FromFormField, ValueField};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Content(String);

impl Content {
    pub fn new(content: &str) -> Result<Self, ClipErr> {
        if !content.trim().is_empty() {
            Ok(Self(content.to_owned()))
        } else {
            Err(ClipErr::EmptyContent)
        }
    }

    // NOTE Common pattern for returning a reference to a field's inner value
    pub fn into_inner(self) -> String {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for Content {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        let res = Self::new(field.value);
        // NOTE map_err() is used to convert the error type from ClipErr to form::Error
        Ok(res.map_err(|e| form::Error::validation(format!("{}", e)))?)
    }
}
