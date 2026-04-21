//! Pages module — reinhardt::pages router
// Workaround: kent8192/reinhardt-web#3866 — startproject generates raw wasm-bindgen instead of PagesRouter
use reinhardt::pages::prelude::*;

pub mod posts_page;

pub fn router() -> PagesRouter {
    PagesRouter::new()
        // .route("/", posts_page::posts_page)
}
