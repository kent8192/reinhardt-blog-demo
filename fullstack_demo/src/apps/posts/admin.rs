//! Admin module for posts app
use reinhardt::admin;
use super::models::Post;

#[admin(model, for = Post, name = "Post", list_display = [id, title, published, created_at])]
pub struct PostAdmin;
