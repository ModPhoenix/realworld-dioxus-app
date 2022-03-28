use dioxus::prelude::*;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TagsResponse {
    tags: Vec<String>,
}

pub fn Tags(cx: Scope) -> Element {
    let tags_response = use_future(&cx, (), |_| async move {
        reqwest::get("https://api.realworld.io/api/tags")
            .await
            .unwrap()
            .json::<TagsResponse>()
            .await
    });

    cx.render(rsx! (
            div { class: "sidebar",
                p { "Popular Tags" }
                div { class: "tag-list",
                match tags_response.value() {
                    Some(Ok(data)) => {
                        if data.tags.len() > 0 {
                            rsx!(data.tags.iter().map(|tag| rsx!(
                                a { key: "{tag}", class: "tag-pill tag-default", href: "", "{tag}" }
                            )))
                        } else {
                            rsx! { div { "No tags" } }
                        }},
                    Some(Err(_)) => rsx! { div { "loading tags failed" } },
                    None => rsx! { div { "loading..." } },
                }
            }
        }
    ))
}
