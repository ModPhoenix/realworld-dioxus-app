use dioxus::{events::FormEvent, fermi::use_atom_root, prelude::*};

use crate::{
    components::FormErrorMessages,
    services::{
        api::API,
        auth_service::{auth_service, AuthService, SIGN_IN_ERROR},
    },
    types::{LoginUser, LoginUserRequest},
};

pub fn SignInForm(cx: Scope) -> Element {
    let router = use_router(&cx);
    let atoms = use_atom_root(&cx);
    let api = cx.consume_context::<API>()?;
    let auth = use_coroutine(&cx, |rx| {
        auth_service(rx, api, atoms.clone(), router.clone())
    });
    let error = use_read(&cx, SIGN_IN_ERROR);
    let set_error = use_set(&cx, SIGN_IN_ERROR);

    cx.use_hook(|_| set_error(None));

    let onsubmit = move |evt: FormEvent| {
        auth.send(AuthService::SignIn(LoginUserRequest {
            user: LoginUser {
                email: evt.values["email"].clone(),
                password: evt.values["password"].clone(),
            },
        }));
    };

    cx.render(rsx!(
        FormErrorMessages { error: error }
        form {
            onsubmit: onsubmit,
            prevent_default: "onsubmit",
            fieldset { class: "form-group",
                input {
                    name: "email",
                    class: "form-control form-control-lg",
                    r#type: "text",
                    placeholder: "Email",
                    required: "true",
                }
            }
            fieldset { class: "form-group",
                input {
                    name: "password",
                    class: "form-control form-control-lg",
                    r#type: "password",
                    placeholder: "Password",
                    required: "true",
                }
            }
            button {
                class: "btn btn-lg btn-primary pull-xs-right",
                "Sign in"
            }
    }))
}
