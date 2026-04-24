//! posts - Shared error types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostsError {
	pub message: String,
}

impl From<String> for PostsError {
	fn from(message: String) -> Self {
		PostsError { message }
	}
}

impl From<&str> for PostsError {
	fn from(message: &str) -> Self {
		PostsError {
			message: message.to_string(),
		}
	}
}
