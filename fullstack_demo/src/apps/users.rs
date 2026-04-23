//! users application module (server-only)

use reinhardt::app_config;

pub mod admin;
pub mod models;
pub mod serializers;
pub mod urls;

#[app_config(name = "users", label = "users")]
pub struct UsersConfig;
