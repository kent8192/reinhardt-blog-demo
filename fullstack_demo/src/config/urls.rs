//! URL configuration for fullstack_demo project (Pages)
//!
//! The `routes` function defines all URL patterns for this project.

use reinhardt::prelude::*;
use reinhardt::routes;

#[routes]
pub fn routes() -> UnifiedRouter {
    UnifiedRouter::new()
        .server(|r| r.viewset("/posts", crate::apps::posts::views::viewset()))
}
