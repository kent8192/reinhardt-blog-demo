//! WASM client for fullstack_demo

mod bootstrap;
mod router;

pub mod pages;


pub use router::with_router;
pub use state::*;
