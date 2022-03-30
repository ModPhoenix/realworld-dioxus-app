#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod features;
mod settings;
mod types;

use crate::{
    features::{
        auth::{SignInPage, SignUpPage},
        home::HomePage,
    },
    settings::path,
};

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    dioxus::web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! (
        Router {
            Route { to: path::HOME, HomePage {} }
            Route { to: path::SIGN_UP, SignUpPage {} }
            Route { to: path::SIGN_IN, SignInPage {} }
        }
    ))
}
