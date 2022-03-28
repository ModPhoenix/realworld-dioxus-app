#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod features;

use crate::{
    components::{Footer, Header},
    features::{auth::SignInPage, home::HomePage},
};

fn main() {
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        Router {
            Header {  }
            Route { to: "/", HomePage {} }
            Route { to: "/sign-in", SignInPage {} }
            Footer {  }
        }
    ))
}
