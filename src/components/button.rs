//! A button component.

use crate::classes::*;
use dioxus::prelude::*;

/// A button style.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ButtonStyle {
    /// Primary button style.
    #[default]
    Primary,
    /// Secondary button style.
    Secondary,
}

/// Generic button component.
#[component]
pub fn Button(
    /// The text on the button.
    text: String,
    /// An optional class name for the loading element.
    class: Option<String>,
    /// The button style.
    style: Option<ButtonStyle>,
    /// Whether the button is disabled.
    #[props(default = false)]
    disabled: bool,
    /// The click event handler.
    onclick: Option<EventHandler<()>>,
) -> Element {
    let button_class = classes!(
        "button",
        match style {
            Some(ButtonStyle::Primary) => Some("primary"),
            Some(ButtonStyle::Secondary) => Some("secondary"),
            None => None,
        },
        class
    );

    rsx! {
        button {
            r#type: "button",
            class: "{button_class}",
            disabled: disabled,
            onclick: move |_| {
                if let Some(onclick) = onclick {
                    onclick.call(())
                }
            },

            "{text}"
        }
    }
}
