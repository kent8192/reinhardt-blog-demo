//! Posts server functions
//!
//! The `#[server_fn]` macro emits an HTTP client stub under WASM and the
//! full implementation on the server. Both share this module path so the
//! WASM client can call the functions directly.
use reinhardt::pages::server_fn::{server_fn, ServerFnError};

use crate::shared::types::{PostSerializer, PublishInput};

#[server_fn]
pub async fn list_posts(
    #[inject] db: reinhardt::DatabaseConnection,
) -> std::result::Result<Vec<PostSerializer>, ServerFnError> {
    use crate::apps::posts::models::Post;
    use reinhardt::Model;

    let rows: Vec<Post> = Post::objects()
        .all()
        .all_with_db(&db)
        .await
        .map_err(|e| ServerFnError::application(e.to_string()))?;
    Ok(rows
        .into_iter()
        .map(|p| PostSerializer {
            id: p.id,
            title: p.title,
            body: p.body,
            published: p.published,
            created_at: p.created_at,
        })
        .collect())
}
