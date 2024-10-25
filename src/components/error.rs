//! UI component for displaying error messages.

use dioxus::prelude::*;

/// An error message component.
#[component]
pub fn Error(
    /// A description of the error.
    description: String,
    /// The error message itself.
    message: String,
) -> Element {
    rsx! {
        div {
            class: "error",

            div {
                class: "error-description",

                "{description}"
            }

            div {
                class: "error-message",

                "{message}"
            }
        }
    }
}
