#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod features;

use crate::{
    components::{footer::Footer, header::Header},
    features::{auth::sign_in_page::SignInPage, home::home_page::HomePage},
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
