use dioxus::prelude::*;
use serde::Deserialize;

use crate::services::api::API;

#[derive(Deserialize)]
pub struct TagsResponse {
    tags: Vec<String>,
}

pub fn Tags(cx: Scope) -> Element {
    let api = cx.consume_context::<API>()?;

    let tags_response = use_future(&cx, (), |_| async move {
        api.client
            .get(API::create_url("/tags"))
            .send()
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
