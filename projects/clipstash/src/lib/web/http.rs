use crate::{
    data::AppDatabase,
    service::{self, action},
    web::{ctx, form, renderer::RenderErr, PageErr},
    ServiceErr, Shortcode,
};
use rocket::{
    form::{Contextual, Form},
    http::{Cookie, CookieJar, SameSite},
    response::{content::RawHtml, status, Redirect},
    uri, State,
};

use super::renderer::Renderer;

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(context, &[]))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home]
}

pub mod catcher {
    use rocket::{catch, catchers, Catcher, Request};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {req:?}");
        "something went wrong..."
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("Internal error: {req:?}");
        "internal error"
    }

    #[catch(404)]
    fn not_found(req: &Request) -> &'static str {
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, internal_error, default]
    }
}
