pub mod data;
pub mod domain;
pub mod service;
pub mod web;

// NOTE re-exporting common modules
pub use domain::clip::field::Shortcode;
pub use domain::clip::ClipErr;
pub use domain::time::Time;
