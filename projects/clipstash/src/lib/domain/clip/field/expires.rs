use std::str::FromStr;

use super::ClipErr;
use crate::domain::time::Time;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Expires(Option<Time>);
impl Expires {
    pub fn new<T: Into<Option<Time>>>(expires: T) -> Self {
        Self(expires.into())
    }

    pub fn into_inner(self) -> Option<Time> {
        self.0
    }
}

impl Default for Expires {
    fn default() -> Self {
        Self::new(None)
    }
}

impl FromStr for Expires {
    type Err = ClipErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // NOTE transpose() maps Ok(Some(x)) to Some(Ok(x))
        let time = (!s.is_empty()).then(|| Time::from_str(s)).transpose()?;
        Ok(Self(time))
    }
    // ? from_str() is equivalent to this:
    // * let time = if !s.is_empty() {
    // *     Some(Time::from_str(s)?)
    // * } else {
    // *     None
    // * };
    // * Ok(Self(time))
}
