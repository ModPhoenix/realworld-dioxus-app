use dioxus::prelude::*;

use crate::settings::path;

#[derive(Props)]
pub struct NavItemProps<'a> {
    label: &'a str,
    to: &'a str,
    #[props(optional)]
    icon: Option<Element<'a>>,
}

pub fn NavItem<'a>(cx: Scope<'a, NavItemProps<'a>>) -> Element {
    let route = use_route(&cx);
    let nav_link_class: &mut String = cx.use_hook(|_| "nav-link".to_string());

    let is_path_match = route.url().path() == cx.props.to;

    if is_path_match {
        *nav_link_class = "nav-link active".to_string();
    } else {
        *nav_link_class = "nav-link".to_string();
    }

    cx.render(rsx! {
        li { class: "nav-item",
            Link {
                class: nav_link_class,
                to: &cx.props.to,
                &cx.props.icon,
                "{cx.props.label}"
            }
        }
    })
}

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! (
        nav { class: "navbar navbar-light",
            div { class: "container",
                Link {
                    class: "navbar-brand",
                    to: path::HOME,
                    "conduit"
                }
                ul { class: "nav navbar-nav pull-xs-right",
                    NavItem {
                        label: "Home",
                        to: path::HOME
                    }
                    NavItem {
                        label: " New Article",
                        to: path::NEW_ARTICLE
                        icon: cx.render(rsx!( i { class: "ion-compose"} ))
                    }
                    NavItem {
                        label: " Settings",
                        to: path::SETTINGS
                        icon: cx.render(rsx!( i { class: "ion-gear-a"} ))
                    }
                    NavItem {
                        label: "Sign in",
                        to: path::SIGN_IN
                    }
                    NavItem {
                        label: "Sign up",
                        to: path::SIGN_UP
                    }
                }
            }
        }
    ))
}
