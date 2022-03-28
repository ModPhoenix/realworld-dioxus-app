use dioxus::prelude::*;

use crate::settings::path;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! (
        nav { class: "navbar navbar-light",
            div { class: "container",
                Link {
                    class: "navbar-brand",
                    to: path::HOME,
                    "conduit"
                }
                ul { class: "nav navbar-nav pull-xs-right",
                    li { class: "nav-item",
                        Link {
                            class: "nav-link active",
                            to: path::HOME,
                            "Home"
                        }
                    }
                    li { class: "nav-item",
                        Link {
                            class: "nav-link",
                            to: path::HOME,
                            i { class: "ion-compose"}
                            " New Article"
                        }
                    }
                    li { class: "nav-item",
                        Link {
                            class: "nav-link",
                            to: path::HOME,
                            i { class: "ion-gear-a"}
                            " Settings"
                        }
                    }
                    li { class: "nav-item",
                        Link {
                            class: "nav-link",
                            to: path::SIGN_IN,
                            "Sign in"
                        }
                    }
                    li { class: "nav-item",
                        Link {
                            class: "nav-link",
                            to: path::SIGN_UP,
                            "Sign up"
                        }
                    }
                }
            }
        }
    ))
}