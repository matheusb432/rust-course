use crate::{
    data::AppDatabase,
    service::{self, action},
    web::{hitcounter::HitCounter, PASSWORD_COOKIE},
    ServiceErr,
};
use rocket::{
    http::{CookieJar, Status},
    request::{FromRequest, Outcome, Request},
    serde::json::Json,
    Responder, State,
};
use serde::Serialize;
use std::str::FromStr;

pub const API_KEY_HEADER: &str = "x-api-key";

#[derive(Responder, Debug, thiserror::Error, Serialize)]
pub enum ApiKeyErr {
    #[error("api key not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(String),
    #[error("invalid api key format")]
    #[response(status = 400, content_type = "json")]
    Decode(String),
}

#[derive(Debug, Clone)]
pub struct ApiKey(Vec<u8>);

impl ApiKey {
    pub fn new() -> Self {
        let key = (0..16).map(|_| rand::random::<u8>()).collect();
        Self(key)
    }

    pub fn to_base64(&self) -> String {
        base64::encode(self.0.as_slice())
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl Default for ApiKey {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for ApiKey {
    type Err = ApiKeyErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        base64::decode(s)
            .map(Self)
            .map_err(|e| ApiKeyErr::Decode(e.to_string()))
    }
}

#[derive(Responder, Debug, thiserror::Error)]
pub enum ApiErr {
    #[error("not found")]
    #[response(status = 404, content_type = "json")]
    NotFound(Json<String>),
    #[error("server error")]
    #[response(status = 500, content_type = "json")]
    Server(Json<String>),
    #[error("client error")]
    #[response(status = 401, content_type = "json")]
    User(Json<String>),
    #[error("invalid api key")]
    #[response(status = 400, content_type = "json")]
    Key(Json<ApiKeyErr>),
}

impl From<ServiceErr> for ApiErr {
    fn from(err: ServiceErr) -> Self {
        match err {
            ServiceErr::Clip(c) => Self::User(Json(format!("clip parsing error: {c}"))),
            ServiceErr::NotFound => Self::NotFound(Json("entity not found".to_string())),
            ServiceErr::Data(_) => Self::Server(Json("internal server error".to_string())),
            ServiceErr::PermissionErr(msg) => Self::User(Json(msg)),
        }
    }
}
