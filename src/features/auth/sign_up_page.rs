use dioxus::prelude::*;

use crate::{
    features::auth::{AuthLayout, SignUpForm},
    settings::path,
};

pub fn SignUpPage(cx: Scope) -> Element {
    cx.render(rsx!(
        AuthLayout {
            title: "Sign up",
            label: "Have an account?",
            to: path::SIGN_IN,
            SignUpForm {}
        }
    ))
}
