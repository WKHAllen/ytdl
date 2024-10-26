//! Text input component.

use crate::classes::*;
use crate::components::ControlError;
use crate::hooks::*;
use dioxus::prelude::*;

/// A text input component.
#[component]
pub fn TextInput(
    /// The input state.
    state: Signal<String>,
    /// The input label.
    label: Option<String>,
    /// The input placeholder text.
    placeholder: Option<String>,
    /// The maximum number of characters allowed.
    #[props(default = 524288)]
    max_length: usize,
    /// Whether the input must be filled out.
    #[props(default = false)]
    required: bool,
    /// Whether the input is disabled.
    #[props(default = false)]
    disabled: bool,
    /// An optional error message.
    #[props(!optional, default)]
    error: Option<String>,
) -> Element {
    let id = use_id();
    let label_text = label.unwrap_or_default();
    let placeholder_text = placeholder.unwrap_or_default();
    let required_mark = required.then_some(" *").unwrap_or_default();
    let container_class = classes!(
        "text-input-container",
        disabled.then_some("text-input-container-disabled")
    );
    let input_class = classes!("text-input", error.as_ref().map(|_| "text-input-invalid"));

    rsx! {
        div {
            class: "{container_class}",

            label {
                class: "text-input-label",
                r#for: "{id}",

                "{label_text}"

                span {
                    class: "required-mark",

                    "{required_mark}"
                }
            }

            input {
                id: "{id}",
                class: "{input_class}",
                r#type: "text",
                placeholder: "{placeholder_text}",
                maxlength: "{max_length}",
                required: "{required}",
                disabled: "{disabled}",
                value: "{state}",
                oninput: move |event| state.set(event.value())
            }

            ControlError {
                message: error
            }
        }
    }
}
