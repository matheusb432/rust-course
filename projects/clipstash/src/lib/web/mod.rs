pub mod api;
pub mod ctx;
pub mod form;
pub mod hitcounter;
pub mod http;
pub mod renderer;

pub const PASSWORD_COOKIE: &'static str = "password";

#[derive(rocket::Responder)]
pub enum PageErr {
    #[response(status = 500)]
    Serialization(String),
    #[response(status = 500)]
    Render(String),
    #[response(status = 404)]
    NotFound(String),
    #[response(status = 500)]
    Internal(String),
}

impl From<handlebars::RenderError> for PageErr {
    fn from(err: handlebars::RenderError) -> Self {
        PageErr::Render(format!("{}", err))
    }
}

impl From<serde_json::Error> for PageErr {
    fn from(err: serde_json::Error) -> Self {
        PageErr::Serialization(format!("{}", err))
    }
}
