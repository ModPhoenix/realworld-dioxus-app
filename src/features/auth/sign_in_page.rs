use dioxus::prelude::*;

pub fn SignInPage(cx: Scope) -> Element {
    cx.render(rsx!(div { class: "auth-page", "SignInPage" }))
}
