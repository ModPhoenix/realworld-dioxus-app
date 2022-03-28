use dioxus::prelude::*;

use crate::features::home::{article_preview::ArticlePreview, tags::Tags};

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! (
        div { class: "home-page",
            div { class: "banner",
                div { class: "container",
                    h1 { class: "logo-font", "conduit"}
                    p { "A place to share your knowledge."}
                }
            }

            div { class: "container page",
                div { class: "row",
                    div { class: "col-md-9",
                        div { class: "feed-toggle",
                            ul { class: "nav nav-pills outline-active",
                                li { class: "nav-item",
                                    a { class: "nav-link disabled", href: "", "Your Feed" }
                                }
                                li { class: "nav-item",
                                    a { class: "nav-link active", href: "", "Global Feed" }
                                }
                            }
                        }

                        ArticlePreview { }
                        ArticlePreview { }
                    }
                    div { class: "col-md-3",
                        Tags {}
                    }
                }
            }
        }
    ))
}
