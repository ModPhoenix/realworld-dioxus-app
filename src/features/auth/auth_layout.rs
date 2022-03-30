use dioxus::prelude::*;

use crate::components::Layout;

#[derive(Props)]
pub struct AuthLayoutProps<'a> {
    title: &'a str,
    label: &'a str,
    to: &'a str,
    children: Element<'a>,
}

pub fn AuthLayout<'a>(cx: Scope<'a, AuthLayoutProps<'a>>) -> Element {
    cx.render(rsx!(
        Layout {
            div { class: "auth-page",
                div { class: "container page",
                    div { class: "row",
                        div { class: "col-md-6 offset-md-3 col-xs-12",
                            h1 { class: "text-xs-center", "{cx.props.title}" },
                            p { class: "text-xs-center",
                                Link { to: cx.props.to, "{cx.props.label}" }
                            }
                            &cx.props.children
                        }
                    }
                }
        }
    }))
}
