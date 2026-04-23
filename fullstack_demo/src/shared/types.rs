//! Shared types used by both client and server
//!
//! These types are serializable and can be sent between the WASM client
//! and the Rust server via server functions.

use serde::{Deserialize, Serialize};

/// Serialized representation of a `Post` crossing the client/server boundary.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostSerializer {
    pub id: i64,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Input payload for [`crate::server_fn::posts::publish_post`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishInput {
    pub id: i64,
}
