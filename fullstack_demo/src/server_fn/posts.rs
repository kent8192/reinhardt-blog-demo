//! Posts server functions
//!
//! The `#[server_fn]` macro emits an HTTP client stub under WASM and the
//! full implementation on the server. Both share this module path so the
//! WASM client can call the functions directly.
