//! posts application module (server-only)

use reinhardt::app_config;

pub mod admin;
pub mod models;
pub mod serializers;
pub mod urls;
pub mod views;
pub mod repositories;

#[app_config(name = "posts", label = "posts")]
pub struct PostsConfig;
