#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod features;
mod settings;

use crate::{
    components::{Footer, Header},
    features::{
        auth::{SignInPage, SignUpPage},
        home::HomePage,
    },
    settings::path,
};

fn main() {
    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        Router {
            Header {  }
            Route { to: path::HOME, HomePage {} }
            Route { to: path::SIGN_UP, SignUpPage {} }
            Route { to: path::SIGN_IN, SignInPage {} }
            Footer {  }
        }
    ))
}
