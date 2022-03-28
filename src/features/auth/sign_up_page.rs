use dioxus::prelude::*;

use crate::settings::path;

pub fn SignUpPage(cx: Scope) -> Element {
    cx.render(rsx!(div { class: "auth-page",
        div { class: "container page",
            div { class: "row",
                div { class: "col-md-6 offset-md-3 col-xs-12",
                    h1 { class: "text-xs-center", "Sign up" },
                    p { class: "text-xs-center",
                        Link { to: path::SIGN_IN, "Have an account?" }
                    }
                    ul { class: "error-messages",
                        li { "That email is already taken" }
                    }
                    form {
                        fieldset { class: "form-group",
                            input {
                                class: "form-control form-control-lg",
                                r#type: "text",
                                placeholder: "Your Name",
                            }
                        }
                        fieldset { class: "form-group",
                            input {
                                class: "form-control form-control-lg",
                                r#type: "text",
                                placeholder: "Email",
                            }
                        }
                        fieldset { class: "form-group",
                            input {
                                class: "form-control form-control-lg",
                                r#type: "password",
                                placeholder: "Password",
                            }
                        }
                        button {
                            class: "btn btn-lg btn-primary pull-xs-right",
                            "Sign up"
                        }
                    }
                }
            }
        }
    }))
}
