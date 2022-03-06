use dioxus::prelude::*;

use crate::components::article_preview::ArticlePreview;

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
                        div { class: "sidebar",
                            p { "Popular Tags" }
                            div { class: "tag-list",
                                a { class: "tag-pill tag-default", href: "", "programming" }
                                a { class: "tag-pill tag-default", href: "", "javascript" }
                                a { class: "tag-pill tag-default", href: "", "emberjs" }
                                a { class: "tag-pill tag-default", href: "", "react" }
                                a { class: "tag-pill tag-default", href: "", "mean" }
                                a { class: "tag-pill tag-default", href: "", "node" }
                                a { class: "tag-pill tag-default", href: "", "rails" }
                                a { class: "tag-pill tag-default", href: "", "angularjs" }
                            }
                        }
                    }
                }
            }
        }
    ))
}
