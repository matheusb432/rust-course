pub mod field;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Error)]
pub enum ClipErr {
    #[error("invalid password: {0}")]
    InvalidPassword(String),
    #[error("invalid title: {0}")]
    InvalidTitle(String),
    #[error("empty content")]
    EmptyContent,
    #[error("invalid date: {0}")]
    InvalidDate(String),
    #[error("date parse error: {0}")]
    DateParse(#[from] chrono::ParseError),
    #[error("id parse error: {0}")]
    Id(#[from] uuid::Error),
    #[error("hits parse error: {0}")]
    Hits(#[from] std::num::TryFromIntError),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Clip {
    // NOTE serde(skip) is used to prevent the field from being serialized on the Json() call
    #[serde(skip)]
    pub clip_id: field::ClipId,
    pub shortcode: field::Shortcode,
    pub content: field::Content,
    pub title: field::Title,
    pub posted: field::Posted,
    pub expires: field::Expires,
    pub password: field::Password,
    pub hits: field::Hits,
}
