//! Posts page component
use reinhardt::pages::prelude::*;
use crate::server_fn::posts::list_posts;
use crate::shared::types::PostSerializer;

pub fn posts_page() -> Page {
    let load_posts = use_action(|_: ()| async move {
        list_posts().await.map_err(|e| e.to_string())
    });
    load_posts.dispatch(());
    let ls = load_posts.clone();

    page!(|ls: Action<Vec<PostSerializer>, String>| {
        section {
            class: "posts",
            h1 { "Posts" }
            {
                Page::Fragment(ls.result().unwrap_or_default().iter()
                    .map(|p| page!(|title: String, body: String| {
                        li {
                            h2 { { title } }
                            p { { body } }
                        }
                    })(p.title.clone(), p.body.clone()))
                    .collect::<Vec<_>>())
            }
        }
    })(ls)
}
