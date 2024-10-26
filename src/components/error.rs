//! UI component for displaying error messages.

use crate::classes::*;
use dioxus::prelude::*;

/// An error message component.
#[component]
pub fn Error(
    /// A description of the error.
    description: String,
    /// The error message itself.
    message: String,
    /// An optional class name for the loading element.
    class: Option<String>,
) -> Element {
    rsx! {
        div {
            class: classes!("error", class),

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
