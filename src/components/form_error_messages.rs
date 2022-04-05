use dioxus::prelude::*;

use crate::types::GenericError;

#[derive(Props, PartialEq)]
pub struct FormErrorMessagesProps<'a> {
    error: &'a Option<GenericError>,
}

pub fn FormErrorMessages<'a>(cx: Scope<'a, FormErrorMessagesProps<'a>>) -> Element {
    if let Some(err) = &cx.props.error {
        cx.render(rsx! (
            ul { class: "error-messages",
                err.errors.iter().map(|(label, vec)| {
                    rsx!(
                        vec.iter().enumerate().map(|(key, message)| {
                            rsx!(li { key: "{key}", "{label}: {message}" })
                        })
                    )
                })
            }
        ))
    } else {
        None
    }
}
