//! Models module for posts app
//!
//! Use the `#[user]` macro to auto-generate `BaseUser`, `FullUser`,
//! `PermissionsMixin`, and `AuthIdentity` trait implementations for user models.
//!
//! # Example
//!
//! ```rust,ignore
//! use reinhardt::macros::user;
//! use reinhardt::Argon2Hasher;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! #[user(hasher = Argon2Hasher, username_field = "email")]
//! #[model(table_name = "users")]
//! pub struct User {
//!     #[field(primary_key = true)]
//!     pub id: uuid::Uuid,
//!     #[field(max_length = 255, unique = true)]
//!     pub email: String,
//!     #[field(max_length = 255)]
//!     pub password_hash: Option<String>,
//!     #[field]
//!     pub last_login: Option<chrono::DateTime<chrono::Utc>>,
//!     #[field(default = true)]
//!     pub is_active: bool,
//!     #[field(default = false)]
//!     pub is_superuser: bool,
//!     #[field(auto_now_add = true)]
//!     pub created_at: chrono::DateTime<chrono::Utc>,
//! }
//! ```
use chrono::{DateTime, Utc};
use reinhardt::db::associations::ForeignKeyField;
use reinhardt::prelude::*;
use serde::{Deserialize, Serialize};

use crate::apps::users::models::User;

#[model(app_label = "posts", table_name = "posts")]
#[derive(Serialize, Deserialize)]
pub struct Post {
    #[field(primary_key = true)]
    pub id: i64,

    #[rel(foreign_key, on_delete = Cascade, related_name = "posts")]
    pub author: ForeignKeyField<User>,

    #[field(max_length = 200)]
    pub title: String,

    #[field(max_length = 10000)]
    pub body: String,

    #[field(default = false)]
    pub published: bool,

    #[field(auto_now_add = true)]
    pub created_at: DateTime<Utc>,
}

impl Post {
    /// Short preview of the post body (first 140 chars).
    pub fn summary(&self) -> String {
        self.body.chars().take(140).collect()
    }
}
