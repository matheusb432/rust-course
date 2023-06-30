use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize, From)]
pub struct Time(DateTime<Utc>);

impl Time {
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn from_naive_utc(datetime: NaiveDateTime) -> Self {
        Self(DateTime::from_utc(datetime, Utc))
    }
}

impl FromStr for Time {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let datetime = format!("{}T00:00:00Z", s).parse::<DateTime<Utc>>()?;
        Ok(datetime.into())
    }
}
