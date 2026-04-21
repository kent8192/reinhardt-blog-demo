//! Admin module for posts app
use reinhardt::admin;
use super::models::Post;

// Workaround: kent8192/reinhardt-web#3867 — correct syntax: model, for = Type
#[admin(model, for = Post, name = "Post", list_display = [id, title, published, created_at])]
pub struct PostAdmin;
