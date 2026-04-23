//! Serializers module for posts app (RESTful + server-side validation)
use reinhardt::Validate;
use super::models::Post;
use crate::shared::types::{PostSerializer as SharedPost, PublishInput as SharedPublishInput};

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

impl From<PostSerializer> for SharedPost {
    fn from(p: PostSerializer) -> Self {
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

impl From<SharedPublishInput> for PublishInput {
    fn from(i: SharedPublishInput) -> Self {
        Self { id: i.id }
    }
}
