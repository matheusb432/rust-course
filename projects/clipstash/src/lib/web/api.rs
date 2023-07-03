use crate::{
    data::AppDatabase,
    domain::{self},
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey {
    type Error = ApiErr;

    // NOTE API Request guard
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        fn server_error() -> Outcome<ApiKey, ApiErr> {
            Outcome::Failure((
                Status::InternalServerError,
                ApiErr::Server(Json("server error".to_string())),
            ))
        }
        fn key_error(e: ApiKeyErr) -> Outcome<ApiKey, ApiErr> {
            Outcome::Failure((Status::BadRequest, ApiErr::Key(Json(e))))
        }
        match req.headers().get_one(API_KEY_HEADER) {
            None => key_error(ApiKeyErr::NotFound("API key not found".to_owned())),
            Some(key) => {
                let db = match req.guard::<&State<AppDatabase>>().await {
                    Outcome::Success(db) => db,
                    _ => return server_error(),
                };
                let api_key = match ApiKey::from_str(key) {
                    Ok(key) => key,
                    Err(e) => return key_error(e),
                };

                match action::api_key_is_valid(api_key.clone(), db.get_pool()).await {
                    Ok(valid) if valid => Outcome::Success(api_key),
                    Ok(valid) if !valid => {
                        key_error(ApiKeyErr::NotFound("API Key not found".to_owned()))
                    }
                    _ => server_error(),
                }
            }
        }
    }
}

// TODO refactor type alias for result

#[rocket::get("/key")]
pub async fn new_api_key(database: &State<AppDatabase>) -> Result<Json<&str>, ApiErr> {
    let api_key = action::generate_api_key(database.get_pool()).await?;
    println!("Api Key: {}", api_key.to_base64());
    Ok(Json("Api key generated. See logs for details."))
}

#[rocket::get("/<shortcode>")]
pub async fn get_clip(
    shortcode: &str,
    database: &State<AppDatabase>,
    cookies: &CookieJar<'_>,
    hit_counter: &State<HitCounter>,
    // NOTE _api_key is not used but it's needed to trigger the request guard
    _api_key: ApiKey,
) -> Result<Json<domain::Clip>, ApiErr> {
    use crate::domain::clip::field::Password;

    // TODO refactor since it's the same as `http.get_raw_clip` mapping logic
    let req = service::ask::GetClip {
        shortcode: shortcode.into(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .map(|raw_password| Password::new(raw_password.to_string()).ok())
            .flatten()
            .unwrap_or_else(Password::default),
    };
    let clip = action::get_clip(req, database.get_pool()).await?;
    hit_counter.hit(shortcode.into(), 1);

    Ok(Json(clip))
}

#[rocket::post("/", data = "<req>")]
pub async fn new_clip(
    req: Json<service::ask::NewClip>,
    database: &State<AppDatabase>,
    _api_key: ApiKey,
) -> Result<Json<domain::Clip>, ApiErr> {
    let clip = action::new_clip(req.into_inner(), database.get_pool()).await?;
    Ok(Json(clip))
}

#[rocket::put("/", data = "<req>")]
pub async fn update_clip(
    req: Json<service::ask::UpdateClip>,
    database: &State<AppDatabase>,
    _api_key: ApiKey,
) -> Result<Json<domain::Clip>, ApiErr> {
    let clip = action::update_clip(req.into_inner(), database.get_pool()).await?;
    Ok(Json(clip))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes!(get_clip, new_clip, update_clip, new_api_key)
}

pub mod catcher {
    use rocket::serde::json::Json;
    use rocket::{catch, catchers, Catcher, Request};

    #[catch(default)]
    fn default(req: &Request) -> Json<&'static str> {
        eprintln!("General error: {req:?}");
        Json("something went wrong...")
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> Json<&'static str> {
        eprintln!("Internal error: {req:?}");
        Json("internal error")
    }

    #[catch(404)]
    fn not_found(_: &Request) -> Json<&'static str> {
        Json("404")
    }

    #[catch(401)]
    fn request_error(_: &Request) -> Json<&'static str> {
        Json("request error")
    }

    #[catch(400)]
    fn missing_api_key(_: &Request) -> Json<&'static str> {
        Json("API key missing or invalid")
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![
            not_found,
            internal_error,
            request_error,
            missing_api_key,
            default
        ]
    }
}
