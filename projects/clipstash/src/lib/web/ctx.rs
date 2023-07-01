use derive_more::Constructor;
// NOTE All of the context data must be serializable to a hashmap to be sent to the template renderer
use serde::Serialize;

pub trait PageContext {
    /// * The page title
    fn title(&self) -> &str;
    /// * Actual path for the template itself in the `/templates` directory
    fn template_path(&self) -> &str;
    /// * Base template, the layout of the page
    /// * `"base"` is the root layout with headers & footers
    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize)]
// NOTE `{}` is necessary so this serializes properly
#[derive(Default)]
pub struct Home {}

impl PageContext for Home {
    fn template_path(&self) -> &str {
        "home"
    }
    fn title(&self) -> &str {
        "Stash Your Clipboard!"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ViewClip {
    pub clip: crate::domain::Clip,
}

impl PageContext for ViewClip {
    fn template_path(&self) -> &str {
        "clip"
    }
    fn title(&self) -> &str {
        "ViewClip"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct PasswordRequired {
    shortcode: crate::Shortcode,
}

impl PageContext for PasswordRequired {
    fn template_path(&self) -> &str {
        "clip_need_password"
    }
    fn title(&self) -> &str {
        "Enter your password"
    }
}
