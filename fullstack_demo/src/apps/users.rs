//! users application module
//!
//! A Reinhardt Pages application with WASM frontend and server functions

use reinhardt::app_config;

// Server-side modules
pub mod admin;
pub mod models;
pub mod serializers;
pub mod urls;

// Client-side modules
#[cfg(client)]
pub mod client;

// Server-side modules
#[cfg(server)]
pub mod server;

// Shared types (both WASM and server)
pub mod shared;

// Re-export commonly used types
pub use shared::errors::*;
pub use shared::types::*;

// Workaround: kent8192/reinhardt-web#3869 — #[routes] macro expects ws_urls module with ws_url_resolvers
#[cfg(server)]
pub mod ws_urls {
    pub mod ws_url_resolvers {
        #[macro_export]
        macro_rules! __for_each_ws_url_resolver_users {
            ($($tt:tt)*) => {};
        }
        pub use __for_each_ws_url_resolver_users as __for_each_ws_url_resolver;
    }
    pub fn patterns() -> reinhardt::WebSocketRouter {
        reinhardt::WebSocketRouter::new()
    }
}

#[app_config(name = "users", label = "users")]
pub struct UsersConfig;
