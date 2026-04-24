//! fullstack_demo library
//!
//! This is the main library crate for fullstack_demo.

// Server-side modules
#[cfg(server)]
pub mod apps;
#[cfg(server)]
pub mod config;

// Client-side modules
#[cfg(client)]
pub mod client;

// Shared types (both WASM and server)
pub mod shared;

// Cross-cfg: server_fn stubs on WASM, impls on server
pub mod server_fn;


// Re-export commonly used items
#[cfg(server)]
pub use config::settings::get_settings;
#[cfg(server)]
pub use config::urls::routes;
