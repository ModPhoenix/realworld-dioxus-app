use dioxus::prelude::*;

use crate::{
    features::auth::{sign_in_form::SignInForm, AuthLayout},
    settings::path,
};

pub fn SignInPage(cx: Scope) -> Element {
    cx.render(rsx!(
        AuthLayout {
            title: "Sign in",
            label: "Need an account?",
            to: path::SIGN_UP,
            SignInForm {}
        }
    ))
}
