pub mod data;
pub mod domain;
pub mod service;
pub mod web;

// NOTE re-exporting common modules
pub use data::DataErr;
pub use domain::clip::field::Shortcode;
pub use domain::clip::ClipErr;
pub use domain::time::Time;
pub use domain::Clip;
pub use service::ServiceErr;

use data::AppDatabase;
use rocket::{fs::FileServer, Build, Rocket};
use web::{hitcounter::HitCounter, renderer::Renderer};

pub fn rocket(config: RocketConfig) -> Rocket<Build> {
    rocket::build()
        .manage::<AppDatabase>(config.database)
        .manage::<Renderer>(config.renderer)
        .manage::<HitCounter>(config.hit_counter)
        .mount("/", web::http::routes())
        .mount("/api/clip", web::api::routes())
        .mount("/static", FileServer::from("static")) // ? "static" refers to the /static folder in the root of our crate
        .register("/", web::http::catcher::catchers())
        .register("/api/clip", web::api::catcher::catchers())
}

pub struct RocketConfig {
    pub renderer: Renderer<'static>,
    pub database: AppDatabase,
    pub hit_counter: HitCounter,
}
