//! Shared types used by both client and server
//!
//! These types are serializable and can be sent between the WASM client
//! and the Rust server via server functions.

use reinhardt::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PostSerializer {
    #[serde(skip_deserializing)]
    pub id: i64,

    #[validate(length(min = 1, max = 200, message = "title must be 1–200 chars"))]
    pub title: String,

    #[validate(length(min = 1, max = 10000, message = "body must be 1–10000 chars"))]
    pub body: String,

    pub published: bool,

    #[serde(skip_deserializing)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct PublishInput {
    #[validate(range(min = 1, message = "id must be a positive integer"))]
    pub id: i64,
}
