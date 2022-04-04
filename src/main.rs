#![allow(non_snake_case)]

use dioxus::prelude::*;
use services::api::API;

mod components;
mod features;
mod services;
mod settings;
mod types;
mod utils;

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
    cx.use_hook(|_| {
        cx.provide_context(API::new());
    });

    cx.render(rsx! (
        Router {
            Route { to: path::HOME, HomePage {} }
            Route { to: path::SIGN_UP, SignUpPage {} }
            Route { to: path::SIGN_IN, SignInPage {} }
        }
    ))
}
