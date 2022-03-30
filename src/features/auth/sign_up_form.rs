use std::collections::HashMap;

use dioxus::{events::FormEvent, prelude::*};
use serde::Deserialize;

use crate::{settings::path, types::User};

#[derive(Debug, Deserialize)]
struct UserResponse {
    user: User,
}

#[derive(Deserialize, Debug)]
struct GenericError {
    errors: HashMap<String, Vec<String>>,
}

pub fn SignUpForm(cx: Scope) -> Element {
    let router = use_router(&cx);
    // let data = use_state(&cx, || None);
    let error = use_state(&cx, || None);

    let onsubmit = move |evt: FormEvent| {
        cx.spawn({
            let router = router.clone();
            let error = error.clone();
            async move {
                let result = reqwest::Client::new()
                    .post("https://api.realworld.io/api/users")
                    .json(&serde_json::json!({
                        "user": {
                            "username": &evt.values["username"],
                            "email": &evt.values["email"],
                            "password": &evt.values["password"]
                        }
                    }))
                    .send()
                    .await;

                match result {
                    Ok(response) => match response.status() {
                        reqwest::StatusCode::OK => {
                            match response.json::<UserResponse>().await {
                                Ok(res) => {
                                    // data.set(Some(res));
                                    let window = web_sys::window().expect("window");
                                    log::debug!("window: {:#?}", window);
                                    log::debug!("res: {:#?}", res);

                                    let jwt = window
                                        .local_storage()
                                        .unwrap()
                                        .unwrap()
                                        .set_item("jwt", &res.user.token);

                                    match jwt {
                                        Ok(ok) => log::debug!("ok: {:#?}", ok),
                                        Err(err) => log::debug!("err: {:#?}", err),
                                    }

                                    router.push_route(path::HOME, None, None)
                                }
                                Err(error) => log::debug!("error: {:#?}", error),
                            };
                        }
                        reqwest::StatusCode::UNPROCESSABLE_ENTITY => {
                            match response.json::<GenericError>().await {
                                Ok(data) => {
                                    log::debug!("data error {:?}", data);
                                    let window = web_sys::window().expect("window");
                                    let jwt = window
                                        .local_storage()
                                        .unwrap()
                                        .unwrap()
                                        .set_item("jwt", "test");

                                    match jwt {
                                        Ok(ok) => log::debug!("ok: {:#?}", ok),
                                        Err(err) => log::debug!("err: {:#?}", err),
                                    }

                                    error.set(Some(data));
                                }
                                Err(error) => log::debug!("error: {:#?}", error),
                            };
                        }
                        _ => log::debug!("Uh oh! Something unexpected happened"),
                    },
                    Err(_) => log::debug!("Uh oh! Something unexpected happened"),
                }
            }
        });
    };

    let error_messages = match error.get() {
        Some(data) => rsx!(
            ul { class: "error-messages",
                data.errors.iter().map(|(label, vec)| {
                    rsx!(
                        vec.iter().map(|message| {
                            rsx!(li { "{label}: {message}" })
                        })
                    )
                }
                )
            }
        ),
        None => rsx!(div {}),
    };

    cx.render(rsx!(
        error_messages
        form {
            onsubmit: onsubmit,
            prevent_default: "onsubmit",
            fieldset { class: "form-group",
                input {
                    name: "username",
                    class: "form-control form-control-lg",
                    r#type: "text",
                    placeholder: "Your Name",
                    required: "true",
                }
            }
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
                "Sign up"
            }
    }))
}
