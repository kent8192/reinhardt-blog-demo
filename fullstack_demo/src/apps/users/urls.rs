//! URL configuration for users app.
//!
//! Routes can be registered in either of two ways (they merge):
//! 1. Inline here, via `unified_url_patterns` below.
//! 2. In submodules under `urls/` (`server_urls`, `client_urls`, `ws_urls`).

#[cfg(server)]
pub mod server_urls;

#[cfg(client)]
pub mod client_urls;

#[cfg(server)]
pub mod ws_urls;

use reinhardt::url_patterns;
use reinhardt::UnifiedRouter;

use crate::config::apps::InstalledApp;

#[url_patterns(InstalledApp::users, mode = unified)]
pub fn unified_url_patterns() -> UnifiedRouter {
    UnifiedRouter::new()
}
