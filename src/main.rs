#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod features;

use crate::{
    components::{footer::Footer, header::Header},
    features::home::home_page::HomePage,
};

fn main() {
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        Header {  }
        HomePage {  }
        Footer {  }
    ))
}
