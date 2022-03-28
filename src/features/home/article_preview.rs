use dioxus::prelude::*;

pub fn ArticlePreview(cx: Scope) -> Element {
    cx.render(rsx! (
        div { class: "article-preview",
            div { class: "article-meta",
                a { class: "navbar-brand", href: "", img { src: "http://i.imgur.com/Qr71crq.jpg" } }
                div { class: "info",
                    a { class: "author", href: "", "Eric Simons" }
                    span { class: "date", "January 20th" }
                }
                button { class: "btn btn-outline-primary btn-sm pull-xs-right", i { class: "ion-heart" } " 29" }
            }
            a { class: "preview-link", href: "",
                h1 { "How to build webapps that scale" }
                p { "This is the description for the post." }
                span { "Read more..." }
            }
        }
    ))
}
