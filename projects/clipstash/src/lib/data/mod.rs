use std::str::FromStr;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Serialize, From, Display)]
pub struct DbId(Uuid);
impl DbId {
    pub fn new() -> Self {
        Uuid::new_v4().into()
    }

    pub fn nil() -> Self {
        Self(Uuid::nil())
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}
