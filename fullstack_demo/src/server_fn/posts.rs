//! Posts server functions
//!
//! The `#[server_fn]` macro emits an HTTP client stub under WASM and the
//! full implementation on the server. Both share this module path so the
//! WASM client can call the functions directly.

use reinhardt::pages::server_fn::{server_fn, ServerFnError};

use crate::shared::types::{PostSerializer, PublishInput};

/// List all posts.
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

/// Publish a single post by id.
#[server_fn]
pub async fn publish_post(
    input: PublishInput,
    #[inject] db: reinhardt::DatabaseConnection,
) -> std::result::Result<PostSerializer, ServerFnError> {
    use crate::apps::posts::models::Post;
    use reinhardt::Model;

    let mut post = Post::objects()
        .get(input.id)
        .all_with_db(&db)
        .await
        .map_err(|e| ServerFnError::application(e.to_string()))?
        .into_iter()
        .next()
        .ok_or_else(|| ServerFnError::application("Post not found".to_string()))?;
    post.published = true;
    Post::objects()
        .update(&post)
        .await
        .map_err(|e| ServerFnError::application(e.to_string()))?;
    Ok(PostSerializer {
        id: post.id,
        title: post.title,
        body: post.body,
        published: post.published,
        created_at: post.created_at,
    })
}
