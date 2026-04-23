//! Server-side URL patterns for posts.

use reinhardt::url_patterns;
use reinhardt::ServerRouter;

use crate::config::apps::InstalledApp;

#[url_patterns(InstalledApp::posts, mode = server)]
pub fn server_url_patterns() -> ServerRouter {
    ServerRouter::new()
    // Register endpoints here.
    // Example: .endpoint(views::index)
}
