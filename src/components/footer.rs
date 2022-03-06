use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx! (
        div { class: "container",
            a { class: "logo-font", href: "/", "conduit" }
            span { class: "attribution",
                " An interactive learning project from "
                a { href: "https://thinkster.io", "Thinkster" }
                ". Code &amp; design licensed under MIT."
            }
        }
    ))
}
