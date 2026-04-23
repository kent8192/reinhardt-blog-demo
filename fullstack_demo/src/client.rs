//! WASM client for fullstack_demo

mod bootstrap;
mod router;
mod state;
pub mod pages;

pub use router::with_router;
pub use state::*;
