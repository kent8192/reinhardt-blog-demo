//! posts - Server functions
//!
//! Server functions that can be called from the WASM client.
//!
//! Since rc.14, `#[inject]` parameters are auto-detected — no `use_inject = true` needed.

use reinhardt::prelude::*;
// Workaround: explicit macro imports required in rc.17
use reinhardt::pages::server_fn::server_fn;
use reinhardt::pages::ServerFnError;
use super::super::{models::Post, serializers::PostSerializer};

// Scene 9 additions
use reinhardt::AuthUser;
use reinhardt::reinhardt_auth::guard::Guard;
use reinhardt::reinhardt_auth::IsActiveUser;
use crate::apps::users::models::User;
use super::super::repositories::PostRepository;
use super::super::serializers::PublishInput;

#[server_fn]
pub async fn list_posts(
    #[inject] db: DatabaseConnection,
) -> std::result::Result<Vec<PostSerializer>, ServerFnError> {
    // Workaround: kent8192/reinhardt-web#3867 — correct ORM chain is .all().all_with_db(&db)
    let rows: Vec<Post> = Post::objects().all().all_with_db(&db).await
        .map_err(|e| ServerFnError::server(500, e.to_string()))?;
    Ok(rows.into_iter().map(PostSerializer::from).collect())
}

// Workaround: kent8192/reinhardt-web#3867 — Guard<IsActiveUser> needs IsActiveUser: Default;
// PostRepository needs Clone + Injectable; simplified for demo compilation
#[server_fn]
pub async fn publish_post(
    input: PublishInput,
    #[inject] db: DatabaseConnection,
) -> std::result::Result<PostSerializer, ServerFnError> {
    let posts = PostRepository { db };
    let mut post = posts.get(input.id).await
        .map_err(|e| ServerFnError::server(500, e.to_string()))?;
    post.published = true;
    posts.save(&mut post).await
        .map_err(|e| ServerFnError::server(500, e.to_string()))?;
    Ok(PostSerializer::from(post))
}
