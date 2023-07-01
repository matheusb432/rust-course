use crate::data::DbId;
use crate::{ClipErr, Shortcode, Time};
use chrono::{NaiveDateTime, Utc};
use std::convert::TryFrom;
use std::str::FromStr;

// ? sqlx::FromRow is a trait that converts a row from a database into a struct
#[derive(Debug, sqlx::FromRow)]
pub struct Clip {
    // NOTE pub(in crate::data) means that the field is public only within the crate
    pub(in crate::data) clip_id: String,
    pub(in crate::data) shortcode: String,
    pub(in crate::data) content: String,
    pub(in crate::data) title: Option<String>,
    pub(in crate::data) posted: NaiveDateTime,
    pub(in crate::data) expires: Option<NaiveDateTime>,
    pub(in crate::data) password: Option<String>,
    pub(in crate::data) hits: i64,
}

// NOTE implementing a conversion from the database Clip to the domain Clip
impl TryFrom<Clip> for crate::domain::clip::Clip {
    type Error = ClipErr;
    fn try_from(row: Clip) -> Result<Self, Self::Error> {
        use crate::domain::clip::field;

        Ok(Self {
            clip_id: field::ClipId::new(DbId::from_str(row.clip_id.as_str())?),
            shortcode: field::Shortcode::from(row.shortcode),
            content: field::Content::new(row.content.as_str())?,
            title: field::Title::new(row.title)?,
            posted: field::Posted::new(Time::from_naive_utc(row.posted)),
            expires: field::Expires::new(row.expires.map(Time::from_naive_utc)),
            password: field::Password::new(row.password)?,
            hits: field::Hits::new(u64::try_from(row.hits)?),
        })
    }
}
