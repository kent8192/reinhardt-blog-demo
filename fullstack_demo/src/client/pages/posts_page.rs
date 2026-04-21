//! Posts page component
// Workaround: kent8192/reinhardt-web#3866
use reinhardt::pages::prelude::*;
// Workaround: kent8192/reinhardt-web#3868 — module path is server::server_fn
use crate::apps::posts::{serializers::PostSerializer, server::server_fn::list_posts};

pub fn posts_page() -> Page {
    let load_posts = use_action(|_: ()| async move {
        list_posts().await.map_err(|e| e.to_string())
    });
    // Workaround: kent8192/reinhardt-web#3868 — module path is server::server_fn
    let publish = use_action(|id: i64| async move {
        crate::apps::posts::server::server_fn::publish_post(
            crate::apps::posts::serializers::PublishInput { id },
        ).await.map_err(|e| e.to_string())
    });
    load_posts.dispatch(());
    let ls = load_posts.clone();

    page!(|ls: Action<Vec<PostSerializer>, String>| {
        section class: "posts" {
            h1 { "Posts" }
            {
                Page::Fragment(ls.result().unwrap_or_default().iter()
                    .map(|p| page!(|title: String, body: String| {
                        li { h2 { { title } } p { { body } } }
                    })(p.title.clone(), p.body.clone()))
                    .collect::<Vec<_>>())
            }
        }
    })(ls)
}
