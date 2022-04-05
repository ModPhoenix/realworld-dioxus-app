use dioxus::prelude::*;

use crate::components::{Footer, Header};

#[derive(Props)]
pub struct LayoutProps<'a> {
    children: Element<'a>,
}

pub fn Layout<'a>(cx: Scope<'a, LayoutProps<'a>>) -> Element {
    cx.render(rsx! (
        Header {  }
        &cx.props.children
        Footer {  }
    ))
}
