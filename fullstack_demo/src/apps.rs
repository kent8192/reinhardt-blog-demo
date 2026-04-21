//! Application registry for fullstack_demo
//!
//! This file maintains the list of installed apps.
//! New apps created with `startapp` will be automatically added here.
pub mod posts;
pub use posts::PostsConfig;
pub mod users;
pub use users::UsersConfig;
