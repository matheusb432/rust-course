use crate::domain::clip::field;
use crate::Shortcode;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GetClip {
    pub shortcode: Shortcode,
    pub password: field::Password,
}

impl GetClip {
    pub fn from_raw(shortcode: &str) -> Self {
        Self {
            // ? Equivalent to -> Shortcode::from(shortcode),
            shortcode: shortcode.into(),
            password: field::Password::default(),
        }
    }
}

impl From<Shortcode> for GetClip {
    fn from(shortcode: Shortcode) -> Self {
        Self {
            shortcode,
            password: field::Password::default(),
        }
    }
}

impl From<&str> for GetClip {
    fn from(value: &str) -> Self {
        Self::from_raw(value)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewClip {
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}

use crate::web::form;

impl From<form::NewClip> for NewClip {
    fn from(value: form::NewClip) -> Self {
        Self {
            content: value.content,
            title: value.title,
            expires: value.expires,
            password: value.password,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateClip {
    pub shortcode: field::Shortcode,
    pub content: field::Content,
    pub title: field::Title,
    pub expires: field::Expires,
    pub password: field::Password,
}
