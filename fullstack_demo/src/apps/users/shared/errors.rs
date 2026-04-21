//! users - Shared error types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsersError {
	pub message: String,
}

impl From<String> for UsersError {
	fn from(message: String) -> Self {
		UsersError { message }
	}
}

impl From<&str> for UsersError {
	fn from(message: &str) -> Self {
		UsersError {
			message: message.to_string(),
		}
	}
}
