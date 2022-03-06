#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;

use crate::components::{footer::Footer, header::Header};

fn main() {
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        Header {  }
        Footer {  }
    ))
}
