//! Reinhardt Project Management CLI for polls_project

#[cfg(native)]
use polls_project as _;
#[cfg(native)]
use reinhardt::commands::execute_from_command_line;
#[cfg(native)]
use std::process;

#[cfg(native)]
#[tokio::main]
async fn main() {
	// SAFETY: Called at program start before any spawned tasks.
	unsafe {
		std::env::set_var(
			"REINHARDT_SETTINGS_MODULE",
			"polls_project.config.settings",
		);
	}

	if let Err(e) = execute_from_command_line().await {
		eprintln!("Error: {e}");
		process::exit(1);
	}
}

// On WASM target the management bin is unused; provide a stub main so the
// crate still type-checks under `cargo check --target wasm32-unknown-unknown`.
#[cfg(wasm)]
fn main() {}
