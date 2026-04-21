//! Serializers module for posts app (RESTful)
use reinhardt::prelude::*;
use reinhardt::Validate;
use super::models::Post;

#[derive(Clone, serde::Serialize, serde::Deserialize, Validate)]
pub struct PostSerializer {
    #[serde(skip_deserializing)]
    pub id: i64,

    #[validate(length(min = 1, max = 200, message = "title must be 1–200 chars"))]
    pub title: String,

    #[validate(length(min = 1, max = 10000, message = "body must be 1–10000 chars"))]
    pub body: String,

    pub published: bool,

    #[serde(skip_deserializing)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// From<Post> covers the read path (DB → response).
// Input validation is handled by pre_validate on #[server_fn].
impl From<Post> for PostSerializer {
    fn from(p: Post) -> Self {
        Self {
            id: p.id,
            title: p.title,
            body: p.body,
            published: p.published,
            created_at: p.created_at,
        }
    }
}

#[derive(serde::Deserialize, Validate)]
pub struct PublishInput {
    #[validate(range(min = 1, message = "id must be a positive integer"))]
    pub id: i64,
}
