//! Server functions
//!
//! Server functions that can be called from the WASM client.
//! These functions are compiled for both WASM (client stubs) and server (actual implementation).
//! Server functions callable from both WASM (HTTP stubs) and server (impls).

pub mod posts;
