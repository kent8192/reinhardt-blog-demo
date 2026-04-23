//! fullstack_demo library
//!
//! This is the main library crate for fullstack_demo.

// Server-only: apps (models, views, admin, urls, serializers)
#[cfg(server)]
pub mod apps;

// Server-only: project configuration
#[cfg(server)]
pub mod config;

// Client-only: WASM bootstrap, router, pages
#[cfg(client)]
pub mod client;

// Cross-cfg: shared types + server functions (stubs on WASM, impls on server)
pub mod server_fn;
pub mod shared;

// Re-export commonly used items
#[cfg(server)]
pub use config::settings::get_settings;
#[cfg(server)]
pub use config::urls::routes;
