use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! (
        nav { class: "navbar navbar-light",
            div { class: "container",
                Link {
                    class: "navbar-brand",
                    to: "/",
                    "conduit"
                }
                ul { class: "nav navbar-nav pull-xs-right",
                    li { class: "nav-item",
                        Link {
                            class: "nav-link active",
                            to: "/",
                            "Home"
                        }
                    }
                    li { class: "nav-item",
                        Link {
                            class: "nav-link",
                            to: "/",
                            i { class: "ion-compose"}
                            " New Article"
                        }
                    }
                    li { class: "nav-item",
                        Link {
                            class: "nav-link",
                            to: "/",
                            i { class: "ion-gear-a"}
                            " Settings"
                        }
                    }
                    li { class: "nav-item",
                        Link {
                            class: "nav-link",
                            to: "/sign-in",
                            "Sign in"
                        }
                    }
                    li { class: "nav-item",
                        Link {
                            class: "nav-link",
                            to: "/sign-up",
                            "Sign up"
                        }
                    }
                }
            }
        }
    ))
}
