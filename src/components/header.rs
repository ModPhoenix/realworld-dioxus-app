use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx! (
        nav { class: "navbar navbar-light",
            div { class: "container",
                a { class: "navbar-brand", href: "/", "conduit" }
                ul { class: "nav navbar-nav pull-xs-right",
                    li { class: "nav-item",
                        a { class: "nav-link active", href: "/", "Home" }
                    }
                    li { class: "nav-item",
                        a { class: "nav-link", href: "",
                            i { class: "ion-compose"}
                            " New Article"
                        }
                    }
                    li { class: "nav-item",
                        a { class: "nav-link", href: "",
                            i { class: "ion-gear-a"}
                            " Settings"
                        }
                    }
                    li { class: "nav-item",
                        a { class: "nav-link", href: "", "Sign in" }
                    }
                    li { class: "nav-item",
                        a { class: "nav-link", href: "", "Sign up" }
                    }
                }
            }
        }
    ))
}
