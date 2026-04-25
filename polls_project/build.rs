//! Build script for polls_project.
//!
//! Sets up cfg aliases used throughout the codebase for conditional
//! compilation between the WASM client and the native server.

use cfg_aliases::cfg_aliases;

fn main() {
	// Rust 2024 edition requires explicit check-cfg declarations
	println!("cargo::rustc-check-cfg=cfg(wasm)");
	println!("cargo::rustc-check-cfg=cfg(native)");

	cfg_aliases! {
		wasm: { all(target_family = "wasm", target_os = "unknown") },
		native: { not(all(target_family = "wasm", target_os = "unknown")) },
	}
}
